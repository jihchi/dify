use super::{antialiased, cli, yiq::YIQ};
use anyhow::{anyhow, Context, Result};
use colored::*;
use image::{
    io::Reader as ImageReader, GenericImageView, ImageBuffer, ImageFormat, Pixel, Rgba, RgbaImage,
};

const MAX_YIQ_POSSIBLE_DELTA: f32 = 35215.0;
const RED_PIXEL: Rgba<u8> = Rgba([255, 0, 0, 255]);
const YELLOW_PIXEL: Rgba<u8> = Rgba([255, 255, 0, 255]);

#[derive(Debug, PartialEq)]
pub enum DiffResult {
    Identical(u32, u32),
    BelowThreshold(u32, u32),
    Different(u32, u32),
    OutOfBounds(u32, u32),
    AntiAliased(u32, u32),
}

pub struct RunParams<'a> {
    pub left: &'a str,
    pub right: &'a str,
    pub output: &'a str,
    pub threshold: f32,
    pub output_image_base: Option<cli::OutputImageBase>,
    pub do_not_check_dimensions: bool,
    pub detect_anti_aliased_pixels: bool,
    pub blend_factor_of_unchanged_pixels: Option<f32>,
}

pub fn run(params: &RunParams) -> Result<Option<u32>> {
    let (left_image, right_image): (Result<RgbaImage>, Result<RgbaImage>) = rayon::join(
        || {
            Ok(ImageReader::open(params.left)
                .with_context(|| {
                    format!("failed to open left image \"{}\"", params.left.magenta()).red()
                })?
                .decode()
                .with_context(|| {
                    format!("failed to decode left image \"{}\"", params.left.magenta()).red()
                })?
                .into_rgba())
        },
        || {
            Ok(ImageReader::open(params.right)
                .with_context(|| {
                    format!("failed to open right image \"{}\"", params.right.magenta()).red()
                })?
                .decode()
                .with_context(|| {
                    format!(
                        "failed to decode right image \"{}\"",
                        params.right.magenta()
                    )
                    .red()
                })?
                .into_rgba())
        },
    );

    let (left_image, right_image) = (left_image?, right_image?);
    let left_dimensions = left_image.dimensions();
    let right_dimensions = right_image.dimensions();

    if !params.do_not_check_dimensions && left_dimensions != right_dimensions {
        return Err(anyhow!(format!(
            "dimensions of the left and right image are different, left: {}, right: {}",
            format!("{}x{}", left_dimensions.0, left_dimensions.1).magenta(),
            format!("{}x{}", right_dimensions.0, right_dimensions.1).magenta(),
        )
        .red()));
    };

    let results = get_results(&left_image, &right_image, params);

    let diffs = results.iter().fold(0, |acc, result| match result {
        DiffResult::Identical(_, _)
        | DiffResult::BelowThreshold(_, _)
        | DiffResult::AntiAliased(_, _) => acc,
        DiffResult::Different(_, _) | DiffResult::OutOfBounds(_, _) => acc + 1,
    });

    if diffs > 0 {
        let mut output_image = match params.output_image_base {
            Some(cli::OutputImageBase::LeftImage) => left_image.clone(),
            Some(cli::OutputImageBase::RightImage) => right_image,
            None => ImageBuffer::new(left_dimensions.0, left_dimensions.1),
        };

        for result in results {
            match result {
                DiffResult::Identical(x, y) | DiffResult::BelowThreshold(x, y) => {
                    if let Some(alpha) = params.blend_factor_of_unchanged_pixels {
                        let left_pixel = left_image.get_pixel(x, y);
                        let yiq_y = YIQ::rgb2y(&left_pixel.to_rgb());
                        let rgba_a = left_pixel.channels()[3] as f32;
                        let color =
                            super::blend_semi_transparent_white(yiq_y, alpha * rgba_a / 255.0)
                                as u8;

                        output_image.put_pixel(x, y, Rgba([color, color, color, u8::MAX]));
                    }
                }
                DiffResult::Different(x, y) | DiffResult::OutOfBounds(x, y) => {
                    output_image.put_pixel(x, y, RED_PIXEL);
                }
                DiffResult::AntiAliased(x, y) => {
                    output_image.put_pixel(x, y, YELLOW_PIXEL);
                }
            }
        }

        output_image
            .save_with_format(params.output, ImageFormat::Png)
            .with_context(|| {
                format!("failed to write diff image \"{}\"", params.output.magenta()).red()
            })?;

        return Ok(Some(diffs));
    }

    Ok(None)
}

pub fn get_results(
    left_image: &RgbaImage,
    right_image: &RgbaImage,
    params: &RunParams,
) -> Vec<DiffResult> {
    let threshold = MAX_YIQ_POSSIBLE_DELTA * params.threshold * params.threshold;
    let pixels = left_image.enumerate_pixels();
    let (width, height) = left_image.dimensions();

    let mut results = Vec::with_capacity((width * height) as usize);

    pixels.for_each(|(x, y, left_pixel)| {
        let result = {
            if right_image.in_bounds(x, y) {
                let right_pixel = right_image.get_pixel(x, y);

                if left_pixel == right_pixel {
                    DiffResult::Identical(x, y)
                } else {
                    let left_pixel = YIQ::from_rgba(left_pixel);
                    let right_pixel = YIQ::from_rgba(right_pixel);
                    let delta = left_pixel.squared_distance(&right_pixel);

                    if delta.abs() > threshold {
                        if params.detect_anti_aliased_pixels
                            && (antialiased(&left_image, x, y, width, height, &right_image)
                                || antialiased(&right_image, x, y, width, height, &left_image))
                        {
                            DiffResult::AntiAliased(x, y)
                        } else {
                            DiffResult::Different(x, y)
                        }
                    } else {
                        DiffResult::BelowThreshold(x, y)
                    }
                }
            } else {
                DiffResult::OutOfBounds(x, y)
            }
        };

        results.push(result)
    });

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    const RUN_PARAMS: RunParams = RunParams {
        left: "",
        right: "",
        output: "",
        threshold: 0.05,
        output_image_base: None,
        do_not_check_dimensions: true,
        detect_anti_aliased_pixels: false,
        blend_factor_of_unchanged_pixels: None,
    };

    #[test]
    fn test_zero_width_height() {
        let actual = get_results(&RgbaImage::new(0, 0), &RgbaImage::new(0, 0), &RUN_PARAMS);
        assert_eq!(Vec::<DiffResult>::new(), actual);
    }

    #[test]
    fn test_1_pixel() {
        let actual = get_results(&RgbaImage::new(1, 1), &RgbaImage::new(1, 1), &RUN_PARAMS);
        assert_eq!(vec![DiffResult::Identical(0, 0)], actual);
    }

    #[test]
    fn test_1_different() {
        let mut left = RgbaImage::new(2, 2);
        left.put_pixel(1, 1, YELLOW_PIXEL);
        let actual = get_results(&left, &RgbaImage::new(2, 2), &RUN_PARAMS);

        assert_eq!(
            vec![
                DiffResult::Identical(0, 0),
                DiffResult::Identical(1, 0),
                DiffResult::Identical(0, 1),
                DiffResult::Different(1, 1),
            ],
            actual
        );
    }
}

use crate::cli;
use anyhow::{anyhow, Context, Result};
use colored::*;
use dify::{antialiased, YIQ};
use image::{io::Reader as ImageReader, GenericImageView, ImageFormat, Pixel, Rgba, RgbaImage};

const MAX_YIQ_POSSIBLE_DELTA: f32 = 35215.0;
const RED_PIXEL: Rgba<u8> = Rgba([255, 0, 0, 255]);
const YELLOW_PIXEL: Rgba<u8> = Rgba([255, 255, 0, 255]);

enum DiffResult {
    Identical,
    BelowThreshold,
    Different,
    OutOfBounds,
    AntiAliased,
}

pub fn run(
    left: &str,
    right: &str,
    output_path: &str,
    threshold: f32,
    outpu_image_base: Option<cli::OutputImageBase>,
    do_not_check_dimensions: bool,
    detect_anti_aliased_pixels: bool,
    blend_factor_of_unchanged_pixels: Option<f32>,
) -> Result<Option<u32>> {
    let left_image = ImageReader::open(left)
        .with_context(|| format!("failed to open left image \"{}\"", left.magenta()).red())?
        .decode()
        .with_context(|| format!("failed to decode left image \"{}\"", left.magenta()).red())?
        .into_rgba();

    let right_image = ImageReader::open(right)
        .with_context(|| format!("failed to open right image \"{}\"", right.magenta()).red())?
        .decode()
        .with_context(|| format!("failed to decode right image \"{}\"", right.magenta()).red())?
        .into_rgba();

    let left_dimensions = left_image.dimensions();
    let right_dimensions = right_image.dimensions();

    if !do_not_check_dimensions && left_dimensions != right_dimensions {
        return Err(anyhow!(format!(
            "dimensions of the left and right image are different, left: {}, right: {}",
            format!("{}x{}", left_dimensions.0, left_dimensions.1).magenta(),
            format!("{}x{}", right_dimensions.0, right_dimensions.1).magenta(),
        )
        .red()));
    };

    let threshold = MAX_YIQ_POSSIBLE_DELTA * threshold * threshold;
    let (width, height) = left_dimensions;

    let mut output_image = match outpu_image_base {
        Some(cli::OutputImageBase::LeftImage) => left_image.clone(),
        Some(cli::OutputImageBase::RightImage) => right_image.clone(),
        None => RgbaImage::new(width, height),
    };

    let mut diffs: u32 = 0;

    for (x, y, left_pixel) in left_image.enumerate_pixels() {
        let result = {
            if right_image.in_bounds(x, y) {
                let right_pixel = right_image.get_pixel(x, y);

                if left_pixel == right_pixel {
                    DiffResult::Identical
                } else {
                    let left_pixel = YIQ::from_rgba(left_pixel);
                    let right_pixel = YIQ::from_rgba(right_pixel);
                    let delta = left_pixel.squared_distance(&right_pixel);

                    if delta.abs() > threshold {
                        if detect_anti_aliased_pixels
                            && (antialiased(&left_image, x, y, width, height, &right_image)
                                || antialiased(&right_image, x, y, width, height, &left_image))
                        {
                            DiffResult::AntiAliased
                        } else {
                            DiffResult::Different
                        }
                    } else {
                        DiffResult::BelowThreshold
                    }
                }
            } else {
                DiffResult::OutOfBounds
            }
        };

        match result {
            DiffResult::Identical | DiffResult::BelowThreshold => {
                match blend_factor_of_unchanged_pixels {
                    Some(alpha) => {
                        let yiq_y = YIQ::rgb2y(&left_pixel.to_rgb());
                        let rgba_a = left_pixel.channels()[3] as f32;
                        let color =
                            dify::blend_semi_transparent_white(yiq_y, alpha * rgba_a / 255.0);

                        output_image.put_pixel(
                            x,
                            y,
                            Rgba([color, color, color, (alpha * rgba_a) as u8]),
                        );
                    }
                    None => {}
                }
            }
            DiffResult::Different | DiffResult::OutOfBounds => {
                diffs += 1;
                output_image.put_pixel(x, y, RED_PIXEL);
            }
            DiffResult::AntiAliased => {
                output_image.put_pixel(x, y, YELLOW_PIXEL);
            }
        }
    }

    if diffs > 0 {
        output_image
            .save_with_format(output_path, ImageFormat::Png)
            .with_context(|| {
                format!("failed to write diff image \"{}\"", output_path.magenta()).red()
            })?;

        return Ok(Some(diffs));
    }

    Ok(None)
}

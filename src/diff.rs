use anyhow::{anyhow, Context, Result};
use colored::*;
use dify::{antialiased, yiq::YIQ};
use image::{io::Reader as ImageReader, GenericImageView, ImageFormat, Rgba, RgbaImage};

const MAX_YIQ_POSSIBLE_DELTA: f32 = 35215.0;
const RED_PIXEL: Rgba<u8> = Rgba([255, 0, 0, 255]);
const YELLOW_PIXEL: Rgba<u8> = Rgba([255, 255, 0, 255]);

pub fn run(
    left: &str,
    right: &str,
    output_path: &str,
    threshold: f32,
    diff_based_on_left: bool,
    do_not_check_dimensions: bool,
    include_anti_aliasing: bool,
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

    let mut output_image = if diff_based_on_left {
        left_image.clone()
    } else {
        RgbaImage::new(width, height)
    };

    let mut diffs: u32 = 0;

    for (x, y, left_pixel) in left_image.enumerate_pixels() {
        let mut is_different = false;

        if right_image.in_bounds(x, y) {
            let right_pixel = right_image.get_pixel(x, y);

            if left_pixel == right_pixel {
                continue;
            }

            let left_pixel = YIQ::from_rgba(left_pixel);
            let right_pixel = YIQ::from_rgba(right_pixel);
            let delta = left_pixel.squared_distance(&right_pixel);

            if delta.abs() > threshold {
                if !include_anti_aliasing
                    && (antialiased(&left_image, x, y, width, height, &right_image)
                        || antialiased(&right_image, x, y, width, height, &left_image))
                {
                    output_image.put_pixel(x, y, YELLOW_PIXEL);
                    is_different = false;
                } else {
                    is_different = true;
                }
            }
        } else {
            is_different = true;
        }

        if is_different {
            diffs += 1;
            output_image.put_pixel(x, y, RED_PIXEL);
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

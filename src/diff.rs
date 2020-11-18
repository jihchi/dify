use anyhow::{anyhow, Context, Result};
use colored::*;
use dify::{antialiased, yiq::YIQ};
use image::{io::Reader as ImageReader, GenericImageView, ImageFormat, Pixel, Rgba, RgbaImage};

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
    let left = ImageReader::open(left)
        .with_context(|| format!("failed to open left image \"{}\"", left.magenta()).red())?
        .decode()
        .with_context(|| format!("failed to decode left image \"{}\"", left.magenta()).red())?;

    let right = ImageReader::open(right)
        .with_context(|| format!("failed to open right image \"{}\"", right.magenta()).red())?
        .decode()
        .with_context(|| format!("failed to decode right image \"{}\"", right.magenta()).red())?;

    let left_dimensions = left.dimensions();
    let right_dimensions = right.dimensions();

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

    let mut output = if diff_based_on_left {
        left.to_rgba()
    } else {
        RgbaImage::new(width, height)
    };

    let mut diffs: u32 = 0;

    for x in 0..width {
        for y in 0..height {
            let mut is_different = false;

            if right.in_bounds(x, y) {
                let pixel = (left.get_pixel(x, y), right.get_pixel(x, y));

                if pixel.0 == pixel.1 {
                    continue;
                }

                let rgb = (pixel.0.to_rgb(), pixel.1.to_rgb());
                let yiq = (YIQ::from_rgb(&rgb.0), YIQ::from_rgb(&rgb.1));
                let delta = yiq.0.squared_distance(&yiq.1);

                if delta.abs() > threshold {
                    if !include_anti_aliasing
                        && (antialiased(&left, x, y, width, height, &right)
                            || antialiased(&right, x, y, width, height, &left))
                    {
                        output.put_pixel(x, y, YELLOW_PIXEL);
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
                output.put_pixel(x, y, RED_PIXEL);
            }
        }
    }

    if diffs > 0 {
        output
            .save_with_format(output_path, ImageFormat::Png)
            .with_context(|| {
                format!("failed to write diff image \"{}\"", output_path.magenta()).red()
            })?;

        return Ok(Some(diffs));
    }

    Ok(None)
}

use anyhow::{anyhow, Context, Result};
use colored::*;
use image::{io::Reader as ImageReader, GenericImageView, ImageFormat, Pixel, Rgba, RgbaImage};

use crate::yiq::YIQ;

const MAX_YIQ_POSSIBLE_DELTA: f32 = 35215.0;

pub fn difference(
    left_image_path: &str,
    right_image_path: &str,
    output_image_path: &str,
    threshold: f32,
    mix_output_with_left: bool,
    dont_check_layout: bool,
) -> Result<Option<i32>> {
    let left = ImageReader::open(left_image_path)
        .with_context(|| format!("failed to open left image: {}", left_image_path.magenta()).red())?
        .decode()
        .with_context(|| {
            format!("failed to decode left image: {}", left_image_path.magenta()).red()
        })?;

    let right = ImageReader::open(right_image_path)
        .with_context(|| {
            format!("failed to open right image: {}", right_image_path.magenta()).red()
        })?
        .decode()
        .with_context(|| {
            format!(
                "failed to decode right image: {}",
                right_image_path.magenta()
            )
            .red()
        })?;

    let left_dimensions = left.dimensions();
    let right_dimensions = right.dimensions();

    if !dont_check_layout && left_dimensions != right_dimensions {
        return Err(anyhow!(format!(
            "layout is different, {} vs {}",
            format!("{:?}", left_dimensions).magenta(),
            format!("{:?}", right_dimensions).magenta(),
        )
        .red()));
    };

    let threshold = MAX_YIQ_POSSIBLE_DELTA * threshold * threshold;
    let (width, height) = left_dimensions;

    let mut output = if mix_output_with_left {
        left.to_rgba()
    } else {
        RgbaImage::new(width, height)
    };

    let mut any_difference = false;

    for x in 0..width {
        for y in 0..height {
            let is_pixel_difference;

            if right.in_bounds(x, y) {
                let pixel = (left.get_pixel(x, y), right.get_pixel(x, y));

                if pixel.0 == pixel.1 {
                    continue;
                }

                let rgb = (pixel.0.to_rgb(), pixel.1.to_rgb());
                let yiq = (YIQ::from_rgb(&rgb.0), YIQ::from_rgb(&rgb.1));
                let delta = yiq.0.squared_distance(&yiq.1);

                is_pixel_difference = delta > threshold;
            } else {
                is_pixel_difference = true;
            }

            if is_pixel_difference {
                any_difference = true;
                output.put_pixel(x, y, Rgba([255, 0, 0, 255]));
            }
        }
    }

    if any_difference {
        output.save_with_format(output_image_path, ImageFormat::Png)?;
        return Ok(Some(1));
    }
    Ok(None)
}

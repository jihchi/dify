extern crate image;

use crate::image::{
    io::Reader as ImageReader, GenericImageView, ImageFormat, Pixel, Rgba, RgbaImage,
};
use std::env;
use std::error::Error;

use dify;

const MAX_YIQ_POSSIBLE_DELTA: f32 = 35215.0;

fn help(program: &str) -> Result<(), Box<dyn Error>> {
    println!(
        "Usage: {} <image 1 path> <image 2 path> <diff image path>",
        program
    );
    Ok(())
}

fn difference(
    left_image: &str,
    right_image: &str,
    output_image: &str,
) -> Result<(), Box<dyn Error>> {
    let left = ImageReader::open(left_image)?.decode()?;
    let right = ImageReader::open(right_image)?.decode()?;

    match (left.dimensions(), right.dimensions()) {
        (l_dim, r_dim) if l_dim != r_dim => Err("layout is different".into()),
        (l_dim, _r_dim) => {
            let threshold = MAX_YIQ_POSSIBLE_DELTA * 0.1 * 0.1;
            let (width, height) = l_dim;
            let mut output = RgbaImage::new(width, height);

            for x in 0..width {
                for y in 0..height {
                    let l_pixel = left.get_pixel(x, y);
                    let r_pixel = right.get_pixel(x, y);
                    let l_rgb = l_pixel.to_rgb();
                    let r_rgb = r_pixel.to_rgb();
                    let l_yiq = dify::YIQ::from_rgb(&l_rgb);
                    let r_yiq = dify::YIQ::from_rgb(&r_rgb);

                    let delta = l_yiq.squared_distance(&r_yiq);

                    if delta > threshold {
                        output.put_pixel(x, y, Rgba([255, 0, 0, 255]));
                    }
                }
            }

            output.save_with_format(output_image, ImageFormat::Png)?;

            Ok(())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 | 2 | 3 => help(&args[0]),
        4 => difference(&args[1], &args[2], &args[3]),
        _ => help(&args[0]),
    }
}

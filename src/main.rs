extern crate image;

use crate::image::{
    io::Reader as ImageReader, GenericImageView, ImageFormat, Pixel, Rgba, RgbaImage,
};
use std::error::Error;

use dify;

fn main() -> Result<(), Box<dyn Error>> {
    let left = ImageReader::open("tiger.jpg")?.decode()?;
    let right = ImageReader::open("tiger-2.jpg")?.decode()?;

    match (left.dimensions(), right.dimensions()) {
        (l_dim, r_dim) if l_dim != r_dim => Err("layout is different".into()),
        (l_dim, _r_dim) => {
            let (width, height) = l_dim;
            let mut output = RgbaImage::new(width, height);

            for x in 0..width {
                for y in 0..height {
                    let l_rgb = left.get_pixel(x, y).to_rgb();
                    let r_rgb = right.get_pixel(x, y).to_rgb();

                    let l_yiq = dify::YIQ::from_rgb(&l_rgb);
                    let r_yiq = dify::YIQ::from_rgb(&r_rgb);

                    let delta = l_yiq.square_root_distance(&r_yiq);

                    if delta > 0.0001 {
                        output.put_pixel(x, y, Rgba([255, 0, 0, 255]));
                    }

                    println!(
                        "[{}, {}] L: {:?}, R: {:?}, E: {}",
                        x, y, l_rgb, r_rgb, delta
                    );
                }
            }

            output.save_with_format("diff.png", ImageFormat::Png)?;

            Ok(())
        }
    }
}

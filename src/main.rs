extern crate image;

use crate::image::io::Reader as ImageReader;
use crate::image::GenericImageView;
use std::error::Error;

use dify;

fn main() -> Result<(), Box<dyn Error>> {
    let left = ImageReader::open("tiger.jpg")?.decode()?;
    let right = ImageReader::open("tiger-2.jpg")?.decode()?;

    println!("left dimensions: {:?}", left.dimensions());
    println!("right dimensions: {:?}", right.dimensions());

    for pixel in left.pixels() {
        let (x, y, rgba) = pixel;
        let yiq = dify::YIQ::from_rgb(&[rgba[0], rgba[1], rgba[2]]);

        println!("x: {}, y: {}, rgba: {:?}, yiq: {:?}", x, y, rgba, yiq);
    }

    Ok(())
}

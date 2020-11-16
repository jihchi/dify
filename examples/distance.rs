use anyhow::Result;
use dify::YIQ;
use image;

fn main() -> Result<()> {
    let left = YIQ::from_rgb(&image::Rgb([0, 0, 0]));
    let right = YIQ::from_rgb(&image::Rgb([255, 255, 255]));

    println!(
        "Squared: {}, Square Root: {}",
        left.squared_distance(&right),
        left.square_root_distance(&right)
    );

    Ok(())
}

use anyhow::{anyhow, bail, Context, Result};
use colored::*;
use getopts::Options;
use image::{io::Reader as ImageReader, GenericImageView, ImageFormat, Pixel, Rgba, RgbaImage};
use std::env;

use dify::YIQ;

const MAX_YIQ_POSSIBLE_DELTA: f32 = 35215.0;

fn print_help(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn difference(
    left_image_path: &str,
    right_image_path: &str,
    output_image_path: &str,
    threshold: f32,
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

    if left_dimensions != right_dimensions {
        return Err(anyhow!(format!(
            "layout is different, {:?} vs {:?}",
            left_dimensions, right_dimensions
        )
        .red()));
    };

    let threshold = MAX_YIQ_POSSIBLE_DELTA * threshold * threshold;
    let (width, height) = left_dimensions;
    let mut output = RgbaImage::new(width, height);
    let mut any_difference = false;

    for x in 0..width {
        for y in 0..height {
            let pixel = (left.get_pixel(x, y), right.get_pixel(x, y));
            let rgb = (pixel.0.to_rgb(), pixel.1.to_rgb());
            let yiq = (YIQ::from_rgb(&rgb.0), YIQ::from_rgb(&rgb.1));
            let delta = yiq.0.squared_distance(&yiq.1);

            if delta > threshold {
                any_difference = true;
                output.put_pixel(x, y, Rgba([255, 0, 0, 255]));
            }
        }
    }

    if !any_difference {
        return Ok(None);
    }
    output.save_with_format(output_image_path, ImageFormat::Png)?;
    Ok(Some(1))
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");
    opts.optopt("l", "left", "file path of left image (base)", "FILE");
    opts.optopt("r", "right", "file path of right image (comparing)", "FILE");

    opts.optopt(
        "o",
        "output",
        "file path of diff image (output, .png only). default: diff.png",
        "FILE",
    );

    opts.optopt(
        "t",
        "threshold",
        "threshold of color difference in range [0, 1]. default: 0.1",
        "NUM",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => bail!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_help(&program, opts);
        return Ok(());
    }

    let opt_left = matches.opt_str("l");
    let opt_right = matches.opt_str("r");

    match (opt_left, opt_right) {
        (Some(left_image_path), Some(right_image_path)) => {
            let output = matches.opt_str("o").unwrap_or("diff.png".to_string());
            let threshold = match matches.opt_str("t") {
                Some(opt_threshold) => opt_threshold.parse::<f32>().with_context(|| {
                    format!(
                        "the value of -t/--threshold is invalid: {}",
                        opt_threshold.magenta()
                    )
                    .red()
                }),
                None => Ok(0.1),
            }?;

            match difference(&left_image_path, &right_image_path, &output, threshold) {
                Ok(None) => Ok(()),
                Ok(Some(code)) => std::process::exit(code),
                Err(e) => Err(e),
            }
        }
        (Some(_left_image), None) => bail!("the argument -r/--right is required".red()),
        (None, Some(_right_image)) => bail!("the argument -l/--left is required".red()),
        (None, None) => {
            print_help(&program, opts);
            Ok(())
        }
    }
}

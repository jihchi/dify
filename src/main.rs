extern crate anyhow;
extern crate getopts;
extern crate image;

use crate::image::{
    io::Reader as ImageReader, GenericImageView, ImageFormat, Pixel, Rgba, RgbaImage,
};
use anyhow::{anyhow, bail, Context, Result};
use getopts::Options;
use std::env;

use dify;

const MAX_YIQ_POSSIBLE_DELTA: f32 = 35215.0;

fn print_help(program: &str, opts: Options) {
    let brief = format!("Usage: {}", program);
    print!("{}", opts.usage(&brief));
}

fn difference(
    left_image: &str,
    right_image: &str,
    output_image: &str,
    threshold: f32,
) -> Result<()> {
    let left = ImageReader::open(left_image)?.decode()?;
    let right = ImageReader::open(right_image)?.decode()?;

    match (left.dimensions(), right.dimensions()) {
        (l_dim, r_dim) if l_dim != r_dim => Err(anyhow!("layout is different")),
        (l_dim, _r_dim) => {
            let threshold = MAX_YIQ_POSSIBLE_DELTA * threshold * threshold;
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

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optopt("l", "left", "the file path of left image", "LEFT");
    opts.optopt("r", "right", "the file path of right image", "RIGHT");
    opts.optopt(
        "o",
        "output",
        "the file path of diff image, PNG only. default: diff.png",
        "OUTPUT",
    );
    opts.optopt(
        "t",
        "threshold",
        "the threshold of color difference in range [0, 1]. default: 0.1",
        "THRESHOLD",
    );
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => bail!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_help(&program, opts);
        return Ok(());
    }

    let left = matches.opt_str("l");
    let right = matches.opt_str("r");

    match (left, right) {
        (Some(l), Some(r)) => {
            let output = matches.opt_str("o").unwrap_or("diff.png".to_string());
            let threshold = matches.opt_str("t").unwrap_or("0.1".to_string());
            let threshold = threshold.parse::<f32>().with_context(|| {
                format!("the value of -t/--threshold ({}) is invalid", threshold,)
            })?;

            difference(&l, &r, &output, threshold)
        }
        (Some(_l), None) => bail!("the argument -r/--right is required"),
        (None, Some(_r)) => bail!("the argument -l/--left is required"),
        (None, None) => {
            print_help(&program, opts);
            Ok(())
        }
    }
}

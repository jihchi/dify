use anyhow::{anyhow, Context, Result};
use colored::*;
use getopts::{Matches, Options};
use std::env;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const SHORT_NAME_HELP: &str = "h";
const SHORT_NAME_VERSION: &str = "v";
const SHORT_NAME_DONT_CHECK_DIMENSIONS: &str = "i";
const SHORT_NAME_COPY_IMAGE_AS_BASE: &str = "c";
const SHORT_NAME_OUTPUT_IMAGE_PATH: &str = "o";
const SHORT_NAME_THRESHOLD: &str = "t";
const SHORT_NAME_DETECT_ANTI_ALIASED_PIXELS: &str = "d";
const SHORT_NAME_BLEND_FACTOR_OF_UNCHENGED_PIXELS: &str = "a";
const DEFAULT_PATH_OF_DIFF_IMAGE: &str = "diff.png";

pub enum OutputImageBase {
    LeftImage,
    RightImage,
}

pub struct Cli {
    program: String,
    matches: Matches,
    options: Options,
}

impl Cli {
    pub fn new() -> Result<Self> {
        let args: Vec<String> = env::args().collect();

        let mut options = Options::new();

        options.optflag(SHORT_NAME_HELP, "help", "print this help menu");
        options.optflag(SHORT_NAME_VERSION, "version", "print the version");

        options.optflag(
            SHORT_NAME_DONT_CHECK_DIMENSIONS,
            "ignore-dimensions",
            "don't check image dimensions",
        );

        options.optflagopt(
            SHORT_NAME_BLEND_FACTOR_OF_UNCHENGED_PIXELS,
            "alpha",
            "blending factor of unchanged pixels in the diff output. ranges from 0 for pure white to 1 for original brightness. (default: 0.1)",
            "NUM"
        );

        options.optopt(
            SHORT_NAME_COPY_IMAGE_AS_BASE,
            "copy-image",
            "copies specific image to output as base. (default: left)",
            "{left, right}",
        );

        options.optflag(
            SHORT_NAME_DETECT_ANTI_ALIASED_PIXELS,
            "detect-anti-aliased",
            "detect anti-aliased pixels. (default: false)",
        );

        options.optopt(
            SHORT_NAME_OUTPUT_IMAGE_PATH,
            "output",
            "the file path of diff image, PNG only. (default: diff.png)",
            "OUTPUT",
        );

        options.optopt(
            SHORT_NAME_THRESHOLD,
            "threshold",
            "matching threshold, ranges from 0 to 1, less more precise. (default: 0.1)",
            "NUM",
        );

        match options.parse(&args[1..]) {
            Ok(matches) => Ok(Self {
                program: args[0].clone(),
                matches,
                options,
            }),
            Err(f) => Err(anyhow!(f.to_string())),
        }
    }

    pub fn print_help(&self) {
        let brief = format!("Usage: {} [options] <LEFT> <RIGHT>", self.program);
        print!("{}", self.options.usage(&brief));
    }

    pub fn print_version(&self) {
        println!("{}", VERSION.unwrap_or(""));
    }

    pub fn show_help(&self) -> bool {
        self.matches.opt_present(SHORT_NAME_HELP)
    }

    pub fn show_version(&self) -> bool {
        self.matches.opt_present(SHORT_NAME_VERSION)
    }

    pub fn copy_specific_image_to_output_as_base(&self) -> Result<Option<OutputImageBase>> {
        match self.matches.opt_str(SHORT_NAME_COPY_IMAGE_AS_BASE) {
            Some(value) => match &value.to_lowercase()[..] {
                "left" => Ok(Some(OutputImageBase::LeftImage)),
                "right" => Ok(Some(OutputImageBase::RightImage)),
                unsupported => Err(anyhow!(format!(
                    "-c/--copy-image \"{}\" is not supported, possible values: left, right",
                    unsupported.magenta()
                )
                .red())),
            },
            None => Ok(None),
        }
    }

    pub fn do_not_check_dimensions(&self) -> bool {
        self.matches.opt_present(SHORT_NAME_DONT_CHECK_DIMENSIONS)
    }

    pub fn detect_anti_aliased_pixels(&self) -> bool {
        self.matches
            .opt_present(SHORT_NAME_DETECT_ANTI_ALIASED_PIXELS)
    }

    pub fn blend_factor_of_unchanged_pixels(&self) -> Result<Option<f32>> {
        match self
            .matches
            .opt_str(SHORT_NAME_BLEND_FACTOR_OF_UNCHENGED_PIXELS)
        {
            Some(s) => s
                .parse::<f32>()
                .with_context(|| {
                    format!(
                        "the value of {} is invalid",
                        format!("-a/--alpha {}", s).magenta()
                    )
                    .red()
                })
                .and_then(|n| {
                    if n >= 0.0 && n <= 1.0 {
                        Ok(Some(n))
                    } else {
                        Err(anyhow!(format!(
                            "the value of {} should be in range 0 to 1",
                            format!("-a/--alpha {}", s).magenta()
                        )
                        .red()))
                    }
                }),
            None => Ok(Some(0.1)),
        }
    }

    pub fn get_output_image_path(&self) -> String {
        self.matches
            .opt_str(SHORT_NAME_OUTPUT_IMAGE_PATH)
            .unwrap_or_else(|| DEFAULT_PATH_OF_DIFF_IMAGE.to_string())
    }

    pub fn get_threshold(&self) -> Result<f32> {
        self.matches
            .opt_str(SHORT_NAME_THRESHOLD)
            .map_or(Ok(0.1), |s| {
                s.parse::<f32>().with_context(|| {
                    format!(
                        "the value of {} is invalid",
                        format!("-t/--threshold {}", s).magenta()
                    )
                    .red()
                })
            })
    }

    pub fn get_image_paths_of_left_right_diff(&self) -> Result<(&str, &str)> {
        let left_image = self
            .matches
            .free
            .get(0)
            .with_context(|| format!("the {} argument is missing", "LEFT".magenta()).red())?;

        let right_image = self
            .matches
            .free
            .get(1)
            .with_context(|| format!("the {} argument is missing", "RIGHT".magenta()).red())?;

        Ok((&left_image, &right_image))
    }
}

use anyhow::{anyhow, Context, Result};
use colored::*;
use getopts::{Matches, Options};
use std::env;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const SHORT_NAME_HELP: &str = "h";
const SHORT_NAME_VERSION: &str = "v";
const SHORT_NAME_DONT_CHECK_DIMENSIONS: &str = "d";
const SHORT_NAME_DIFF_BASED_ON_LEFT: &str = "b";
const SHORT_NAME_THRESHOLD: &str = "t";
const SHORT_NAME_DETECT_ANTI_ALIASED_PIXELS: &str = "a";
const DEFAULT_PATH_OF_DIFF_IMAGE: &'static str = "diff.png";

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
            "dont-check-dimensions",
            "don't check image dimensions",
        );

        options.optflag(
            SHORT_NAME_DIFF_BASED_ON_LEFT,
            "diff-based-on-left",
            "draw the diff image based on the left image",
        );

        options.optflag(
            SHORT_NAME_DETECT_ANTI_ALIASED_PIXELS,
            "detect-anti-aliased",
            "detect anti-aliased pixels. default: false",
        );

        options.optopt(
            SHORT_NAME_THRESHOLD,
            "threshold",
            "matching threshold, ranges from 0 to 1, less more precise. default: 0.1",
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
        let brief = format!("Usage: {} [options] LEFT RIGHT [OUTPUT]", self.program);
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

    pub fn diff_based_on_left(&self) -> bool {
        self.matches.opt_present(SHORT_NAME_DIFF_BASED_ON_LEFT)
    }

    pub fn do_not_check_dimensions(&self) -> bool {
        self.matches.opt_present(SHORT_NAME_DONT_CHECK_DIMENSIONS)
    }

    pub fn detect_anti_aliased_pixels(&self) -> bool {
        self.matches
            .opt_present(SHORT_NAME_DETECT_ANTI_ALIASED_PIXELS)
    }

    pub fn get_threshold(&self) -> Result<f32> {
        self.matches
            .opt_str(SHORT_NAME_THRESHOLD)
            .map_or(Ok(0.1), |opt| {
                opt.parse::<f32>().with_context(|| {
                    format!(
                        "the value of {} is invalid",
                        format!("-t/--threshold {}", opt).magenta()
                    )
                    .red()
                })
            })
    }

    pub fn get_image_paths_of_left_right_diff(&self) -> Result<(&str, &str, &str)> {
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

        let diff_image = self
            .matches
            .free
            .get(2)
            .map_or(DEFAULT_PATH_OF_DIFF_IMAGE, String::as_str);

        Ok((&left_image, &right_image, diff_image))
    }
}

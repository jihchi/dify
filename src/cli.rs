use anyhow::{bail, Result};
use getopts::{Matches, Options};
use std::env;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub struct Cli {
    program: String,
    pub matches: Matches,
    options: Options,
}

impl Cli {
    pub fn new() -> Result<Self> {
        let args: Vec<String> = env::args().collect();

        let mut options = Options::new();

        options.optflag("h", "help", "print this help menu");
        options.optflag("m", "mix-output-left", "mix diff image with left image");
        options.optflag("d", "dont-check-layout", "don't check image layout");
        options.optflag("v", "version", "print the version");
        options.optopt("l", "left", "file path of left image (base)", "FILE");
        options.optopt("r", "right", "file path of right image (comparing)", "FILE");

        options.optopt(
            "o",
            "output",
            "file path of diff image (output, .png only). default: diff.png",
            "FILE",
        );

        options.optopt(
            "t",
            "threshold",
            "threshold of color difference in range [0, 1]. default: 0.1",
            "NUM",
        );

        match options.parse(&args[1..]) {
            Ok(matches) => Ok(Self {
                program: args[0].clone(),
                matches,
                options,
            }),
            Err(f) => bail!(f.to_string()),
        }
    }

    pub fn print_help(&self) {
        let brief = format!("Usage: {} [options]", self.program);
        print!("{}", self.options.usage(&brief));
    }

    pub fn print_version(&self) {
        println!("{}", VERSION.unwrap_or(""));
    }
}

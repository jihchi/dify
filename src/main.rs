mod cli;
mod diff;

use anyhow::{bail, Context, Result};
use cli::Cli;
use colored::*;

fn main() -> Result<()> {
    let cli = Cli::new()?;

    if cli.matches.opt_present("h") {
        cli.print_help();
        return Ok(());
    }

    let left_image_path = cli.matches.opt_str("l");
    let right_image_path = cli.matches.opt_str("r");

    match (left_image_path, right_image_path) {
        (Some(left), Some(right)) => {
            let mix_output_with_left = cli.matches.opt_present("m");
            let dont_check_layout = cli.matches.opt_present("d");

            let output = match cli.matches.opt_str("o") {
                Some(s) => s,
                None => "diff.png".to_string(),
            };

            let threshold = match cli.matches.opt_str("t") {
                Some(opt) => opt.parse::<f32>().with_context(|| {
                    format!("the value of -t/--threshold is invalid: {}", opt.magenta()).red()
                }),
                None => Ok(0.1),
            }?;

            match diff::difference(
                &left,
                &right,
                &output,
                threshold,
                mix_output_with_left,
                dont_check_layout,
            ) {
                Ok(None) => Ok(()),
                Ok(Some(code)) => std::process::exit(code),
                Err(e) => Err(e),
            }
        }
        (Some(_left_image), None) => bail!("the argument -r/--right is required".red()),
        (None, Some(_right_image)) => bail!("the argument -l/--left is required".red()),
        (None, None) => {
            cli.print_help();
            Ok(())
        }
    }
}

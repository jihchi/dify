mod cli;
mod diff;

use anyhow::{bail, Result};
use cli::Cli;
use colored::*;

fn main() -> Result<()> {
    let cli = Cli::new()?;

    if cli.show_help() {
        cli.print_help();
        return Ok(());
    }

    if cli.show_version() {
        cli.print_version();
        return Ok(());
    }

    match (cli.get_left_image_path(), cli.get_right_image_path()) {
        (Some(left), Some(right)) => {
            let diff_based_on_left = cli.diff_based_on_left();
            let do_not_check_dimensions = cli.do_not_check_dimensions();
            let output = cli.get_output_image_path();
            let threshold = cli.get_threshold()?;

            diff::run(
                &left,
                &right,
                &output,
                threshold,
                diff_based_on_left,
                do_not_check_dimensions,
            )
            .map(|code| {
                if let Some(code) = code {
                    std::process::exit(code)
                }
                ()
            })
        }
        (Some(_left), None) => {
            bail!(format!("the argument {} is missing", "-r/--right FILE".magenta()).red())
        }
        (None, Some(_right)) => {
            bail!(format!("the argument {} is missing", "-l/--left FILE".magenta()).red())
        }
        (None, None) => {
            cli.print_help();
            Ok(())
        }
    }
}

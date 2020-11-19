mod cli;
mod diff;

use anyhow::Result;
use cli::Cli;

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

    let (left, right, output) = cli.get_image_paths_of_left_right_diff()?;
    let diff_based_on_left = cli.diff_based_on_left();
    let do_not_check_dimensions = cli.do_not_check_dimensions();
    let threshold = cli.get_threshold()?;
    let detect_anti_aliased_pixels = cli.detect_anti_aliased_pixels();

    diff::run(
        &left,
        &right,
        &output,
        threshold,
        diff_based_on_left,
        do_not_check_dimensions,
        detect_anti_aliased_pixels,
    )
    .map(|code| {
        if let Some(code) = code {
            std::process::exit(code as i32)
        }
    })
}

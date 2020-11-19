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

    let (left, right) = cli.get_image_paths_of_left_right_diff()?;
    let output = cli.get_output_image_path();
    let output_image_base = cli.copy_specific_image_to_output_as_base()?;
    let do_not_check_dimensions = cli.do_not_check_dimensions();
    let threshold = cli.get_threshold()?;
    let detect_anti_aliased_pixels = cli.detect_anti_aliased_pixels();

    diff::run(
        &left,
        &right,
        &output,
        threshold,
        output_image_base,
        do_not_check_dimensions,
        detect_anti_aliased_pixels,
    )
    .map(|code| {
        if let Some(code) = code {
            std::process::exit(code as i32)
        }
    })
}

use anyhow::Result;
use dify::{cli::Cli, diff};

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
    let output = &cli.get_output_image_path();
    let output_image_base = cli.copy_specific_image_to_output_as_base()?;
    let do_not_check_dimensions = cli.do_not_check_dimensions();
    let threshold = cli.get_threshold()?;
    let detect_anti_aliased_pixels = cli.detect_anti_aliased_pixels();
    let blend_factor_of_unchanged_pixels = cli.blend_factor_of_unchanged_pixels()?;
    let block_out_areas = cli.get_block_out_area();

    diff::run(&diff::RunParams {
        left,
        right,
        output,
        threshold,
        output_image_base,
        do_not_check_dimensions,
        detect_anti_aliased_pixels,
        blend_factor_of_unchanged_pixels,
        block_out_areas,
    })
    .map(|code| {
        if let Some(code) = code {
            std::process::exit(code)
        }
    })
}

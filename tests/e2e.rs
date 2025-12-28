use assert_cmd::{Command, cargo_bin};
use assert_fs::assert::PathAssert;
use assert_fs::fixture::NamedTempFile;
use predicates::prelude::*;
use std::env::consts;
use std::fs;
use std::path;

#[test]
fn test_sanity() {
    let mut cmd = Command::new(cargo_bin!("dify"));
    cmd.assert().failure();
}

#[test]
fn test_left_argument_is_missing() {
    let mut cmd = Command::new(cargo_bin!("dify"));
    cmd.assert().stderr("Error: the LEFT argument is missing\n");
}

#[test]
fn test_right_argument_is_missing() {
    let mut cmd = Command::new(cargo_bin!("dify"));
    let assert = cmd.arg(path::PathBuf::from("./nonexistent-left.file"));

    assert
        .assert()
        .stderr("Error: the RIGHT argument is missing\n");
}

#[test]
fn test_left_does_not_exist() {
    let mut cmd = Command::new(cargo_bin!("dify"));
    let left = path::PathBuf::from("./nonexistent-left.file");
    let assert = cmd
        .arg(&left)
        .arg(path::PathBuf::from("./nonexistent-right.file"));

    assert.assert().stderr(format!(
        r#"Error: failed to open left image "{}"

Caused by:
    {} (os error 2)
"#,
        left.display(),
        match consts::OS {
            "windows" => "The system cannot find the file specified.",
            "linux" | "macos" => "No such file or directory",
            _ => "<Unsupported operation system>",
        }
    ));
}

#[test]
fn test_right_does_not_exist() {
    let mut cmd = Command::new(cargo_bin!("dify"));
    let right = path::PathBuf::from("./nonexistent-right.file");
    let assert = cmd
        .arg(fs::canonicalize("./benches/fixtures/tiger.jpg").unwrap())
        .arg(&right);

    assert.assert().stderr(format!(
        r#"Error: failed to open right image "{}"

Caused by:
    {} (os error 2)
"#,
        right.display(),
        match consts::OS {
            "windows" => "The system cannot find the file specified.",
            "linux" | "macos" => "No such file or directory",
            _ => "<Unsupported operation system>",
        }
    ));
}

#[test]
fn test_identical_image() {
    let output = NamedTempFile::new("test_identical_image-diff.png")
        .unwrap()
        .into_persistent_if(std::env::var_os("CI").is_some());
    let mut cmd = Command::new(cargo_bin!("dify"));
    let assert = cmd
        .arg(fs::canonicalize("./benches/fixtures/tiger.jpg").unwrap())
        .arg(fs::canonicalize("./benches/fixtures/tiger.jpg").unwrap())
        .arg("--output")
        .arg(output.path().display().to_string());

    assert.assert().success();

    output.close().unwrap();
}

#[test]
fn test_different_image() {
    let output = NamedTempFile::new("test_different_image-diff.png").unwrap();
    let mut cmd = Command::new(cargo_bin!("dify"));
    let assert = cmd
        .arg(fs::canonicalize("./benches/fixtures/tiger.jpg").unwrap())
        .arg(fs::canonicalize("./benches/fixtures/tiger-2.jpg").unwrap())
        .arg("--output")
        .arg(output.path().display().to_string());

    assert.assert().code(match consts::OS {
        "windows" => 7787,
        "linux" | "macos" => 106,
        _ => 106,
    });

    output.close().unwrap();
}

#[test]
fn test_output_image() {
    let running_on_ci = std::env::var_os("CI").is_some();
    let output = NamedTempFile::new("test_output_image-diff.png")
        .unwrap()
        .into_persistent_if(running_on_ci);
    let mut cmd = Command::new(cargo_bin!("dify"));
    let assert = cmd
        .arg(fs::canonicalize("./benches/fixtures/tiger.jpg").unwrap())
        .arg(fs::canonicalize("./benches/fixtures/tiger-2.jpg").unwrap())
        .arg("--output")
        .arg(output.path().display().to_string());

    assert.assert().failure();
    output.assert(predicate::path::eq_file(
        fs::canonicalize("./benches/fixtures/test_output_image-diff.png").unwrap(),
    ));

    output.close().unwrap();
}

#[test]
fn test_output_image_4k() {
    let output = NamedTempFile::new("test_output_image_4k-diff.png")
        .unwrap()
        .into_persistent_if(std::env::var_os("CI").is_some());
    let mut cmd = Command::new(cargo_bin!("dify"));
    let assert = cmd
        .arg(fs::canonicalize("./benches/fixtures/water-4k.png").unwrap())
        .arg(fs::canonicalize("./benches/fixtures/water-4k-2.png").unwrap())
        .arg("--output")
        .arg(output.path().display().to_string());

    assert.assert().failure();
    output.assert(predicate::path::eq_file(
        fs::canonicalize("./benches/fixtures/water-4k-diff.png").unwrap(),
    ));

    output.close().unwrap();
}

#[test]
fn test_output_image_web_page() {
    let output = NamedTempFile::new("test_output_image_web_page-diff.png")
        .unwrap()
        .into_persistent_if(std::env::var_os("CI").is_some());
    let mut cmd = Command::new(cargo_bin!("dify"));
    let assert = cmd
        .arg(fs::canonicalize("./benches/fixtures/www.cypress.io.png").unwrap())
        .arg(fs::canonicalize("./benches/fixtures/www.cypress.io-2.png").unwrap())
        .arg("--output")
        .arg(output.path().display().to_string());

    assert.assert().failure();
    output.assert(predicate::path::eq_file(
        fs::canonicalize("./benches/fixtures/www.cypress.io-diff.png").unwrap(),
    ));

    output.close().unwrap();
}

#[test]
fn test_block_out_area() {
    let running_on_ci = std::env::var_os("CI").is_some();
    let output = NamedTempFile::new("test_block_out_area-diff.png")
        .unwrap()
        .into_persistent_if(running_on_ci);
    let mut cmd = Command::new(cargo_bin!("dify"));
    let assert = cmd
        .arg(fs::canonicalize("./benches/fixtures/tiger.jpg").unwrap())
        .arg(fs::canonicalize("./benches/fixtures/yellow.jpg").unwrap())
        .arg("--output")
        .arg(output.path().display().to_string())
        .arg("--copy-image")
        .arg("left")
        .arg("--block-out")
        .arg("100,50,350,400");

    assert.assert().failure();
    output.assert(predicate::path::eq_file(
        fs::canonicalize("./benches/fixtures/test_block_out_area-diff.png").unwrap(),
    ));

    output.close().unwrap();
}

// https://github.com/jihchi/dify/issues/32
// Test images provided by https://github.com/lucasmerlin
#[test]
fn test_issue_32_transparent_vs_opaque_black() {
    let output = NamedTempFile::new("test_transparent_black.png").unwrap();
    let mut cmd = Command::new(cargo_bin!("dify"));
    let assert = cmd
        .arg(fs::canonicalize("./tests/fixtures/black_transparent.png").unwrap())
        .arg(fs::canonicalize("./tests/fixtures/black_opaque.png").unwrap())
        .arg("--output")
        .arg(output.path().display().to_string());

    assert.assert().code(match consts::OS {
        "windows" => 7787,
        "linux" | "macos" => 212,
        _ => 212,
    });

    output.close().unwrap();
}

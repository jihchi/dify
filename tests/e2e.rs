use assert_cmd::Command;
use assert_fs::assert::PathAssert;
use assert_fs::fixture::NamedTempFile;
use predicates::prelude::*;
use std::fs;
use std::path;

#[test]
fn test_sanity() {
    let mut cmd = Command::cargo_bin("dify").unwrap();
    cmd.assert().failure();
}

#[test]
fn test_left_argument_is_missing() {
    let mut cmd = Command::cargo_bin("dify").unwrap();
    cmd.assert().stderr("Error: the LEFT argument is missing\n");
}

#[test]
fn test_right_argument_is_missing() {
    let mut cmd = Command::cargo_bin("dify").unwrap();
    let assert = cmd.arg(path::PathBuf::from("./nonexistent-left.file"));

    assert
        .assert()
        .stderr("Error: the RIGHT argument is missing\n");
}

#[test]
fn test_left_does_not_exist() {
    let mut cmd = Command::cargo_bin("dify").unwrap();
    let left = path::PathBuf::from("./nonexistent-left.file");
    let assert = cmd
        .arg(&left)
        .arg(path::PathBuf::from("./nonexistent-right.file"));

    assert.assert().stderr(format!(
        r#"Error: failed to open left image "{}"

Caused by:
    No such file or directory (os error 2)
"#,
        left.display().to_string()
    ));
}

#[test]
fn test_right_does_not_exist() {
    let mut cmd = Command::cargo_bin("dify").unwrap();
    let right = path::PathBuf::from("./nonexistent-right.file");
    let assert = cmd
        .arg(fs::canonicalize("./benches/fixtures/tiger.jpg").unwrap())
        .arg(&right);

    assert.assert().stderr(format!(
        r#"Error: failed to open right image "{}"

Caused by:
    No such file or directory (os error 2)
"#,
        right.display().to_string()
    ));
}

#[test]
fn test_identical_image() {
    let mut cmd = Command::cargo_bin("dify").unwrap();
    let assert = cmd
        .arg(fs::canonicalize("./benches/fixtures/tiger.jpg").unwrap())
        .arg(fs::canonicalize("./benches/fixtures/tiger.jpg").unwrap());

    assert.assert().success();
}

#[test]
fn test_different_image() {
    let mut cmd = Command::cargo_bin("dify").unwrap();
    let assert = cmd
        .arg(fs::canonicalize("./benches/fixtures/tiger.jpg").unwrap())
        .arg(fs::canonicalize("./benches/fixtures/tiger-2.jpg").unwrap());

    assert.assert().code(107);
}

#[test]
fn test_output_image() {
    let temp = NamedTempFile::new("tiger-diff.png").unwrap();
    let mut cmd = Command::cargo_bin("dify").unwrap();
    let assert = cmd
        .arg(fs::canonicalize("./benches/fixtures/tiger.jpg").unwrap())
        .arg(fs::canonicalize("./benches/fixtures/tiger-2.jpg").unwrap())
        .arg("--output")
        .arg(temp.path().display().to_string());

    assert.assert().failure();
    temp.assert(predicate::path::eq_file(
        fs::canonicalize("./benches/fixtures/tiger-diff.png").unwrap(),
    ));

    temp.close().unwrap();
}

#[test]
fn test_output_image_4k() {
    let temp = NamedTempFile::new("water-4k-diff.png").unwrap();
    let mut cmd = Command::cargo_bin("dify").unwrap();
    let assert = cmd
        .arg(fs::canonicalize("./benches/fixtures/water-4k.png").unwrap())
        .arg(fs::canonicalize("./benches/fixtures/water-4k-2.png").unwrap())
        .arg("--output")
        .arg(temp.path().display().to_string());

    assert.assert().failure();
    temp.assert(predicate::path::eq_file(
        fs::canonicalize("./benches/fixtures/water-4k-diff.png").unwrap(),
    ));

    temp.close().unwrap();
}

#[test]
fn test_output_image_web_page() {
    let temp = NamedTempFile::new("www.cypress.io-diff.png").unwrap();
    let mut cmd = Command::cargo_bin("dify").unwrap();
    let assert = cmd
        .arg(fs::canonicalize("./benches/fixtures/www.cypress.io.png").unwrap())
        .arg(fs::canonicalize("./benches/fixtures/www.cypress.io-2.png").unwrap())
        .arg("--output")
        .arg(temp.path().display().to_string());

    assert.assert().failure();
    temp.assert(predicate::path::eq_file(
        fs::canonicalize("./benches/fixtures/www.cypress.io-diff.png").unwrap(),
    ));

    temp.close().unwrap();
}

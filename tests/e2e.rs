use assert_cmd::Command;
use assert_fs::assert::PathAssert;
use assert_fs::fixture::NamedTempFile;
use predicates::prelude::*;

const LEFT: &str = "./benches/fixtures/tiger.jpg";
const RIGHT: &str = "./benches/fixtures/tiger-2.jpg";
const OUTPUT: &str = "./benches/fixtures/tiger-diff.png";

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
    let assert = cmd.arg("<left>");

    assert
        .assert()
        .stderr("Error: the RIGHT argument is missing\n");
}

#[test]
fn test_left_does_not_exist() {
    let mut cmd = Command::cargo_bin("dify").unwrap();
    let assert = cmd.arg("<left>").arg("<right>");

    assert.assert().stderr(
        r#"Error: failed to open left image "<left>"

Caused by:
    No such file or directory (os error 2)
"#,
    );
}

#[test]
fn test_right_does_not_exist() {
    let mut cmd = Command::cargo_bin("dify").unwrap();
    let assert = cmd.arg(LEFT).arg("<right>");

    assert.assert().stderr(
        r#"Error: failed to open right image "<right>"

Caused by:
    No such file or directory (os error 2)
"#,
    );
}

#[test]
fn test_identical_image() {
    let mut cmd = Command::cargo_bin("dify").unwrap();
    let assert = cmd.arg(LEFT).arg(LEFT);

    assert.assert().success();
}

#[test]
fn test_different_image() {
    let mut cmd = Command::cargo_bin("dify").unwrap();
    let assert = cmd.arg(LEFT).arg(RIGHT);

    assert.assert().code(107);
}

#[test]
fn test_output_image() {
    let temp = NamedTempFile::new("tiger-diff.png").unwrap();
    let mut cmd = Command::cargo_bin("dify").unwrap();
    let assert = cmd
        .arg(LEFT)
        .arg(RIGHT)
        .arg("--output")
        .arg(temp.path().display().to_string());

    assert.assert().failure();
    temp.assert(predicate::path::eq_file(std::path::Path::new(OUTPUT)));

    temp.close().unwrap();
}

#[test]
fn test_output_image_4k() {
    let temp = NamedTempFile::new("water-4k-diff.png").unwrap();
    let mut cmd = Command::cargo_bin("dify").unwrap();
    let assert = cmd
        .arg("./benches/fixtures/water-4k.png")
        .arg("./benches/fixtures/water-4k-2.png")
        .arg("--output")
        .arg(temp.path().display().to_string());

    assert.assert().failure();
    temp.assert(predicate::path::eq_file(std::path::Path::new(
        "./benches/fixtures/water-4k-diff.png",
    )));

    temp.close().unwrap();
}

#[test]
fn test_output_image_web_page() {
    let temp = NamedTempFile::new("www.cypress.io-diff.png").unwrap();
    let mut cmd = Command::cargo_bin("dify").unwrap();
    let assert = cmd
        .arg("./benches/fixtures/www.cypress.io.png")
        .arg("./benches/fixtures/www.cypress.io-2.png")
        .arg("--output")
        .arg(temp.path().display().to_string());

    assert.assert().failure();
    temp.assert(predicate::path::eq_file(std::path::Path::new(
        "./benches/fixtures/www.cypress.io-diff.png",
    )));

    temp.close().unwrap();
}

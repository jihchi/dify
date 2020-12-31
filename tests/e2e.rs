use assert_cmd::Command;

#[test]
fn test_sanity() {
    let mut cmd = Command::cargo_bin("dify").unwrap();
    cmd.assert().failure();
}

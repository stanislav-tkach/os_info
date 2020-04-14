use assert_cmd::Command;

#[test]
fn no_args() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .expect("cargo_bin failed")
        .assert()
        .success();
}

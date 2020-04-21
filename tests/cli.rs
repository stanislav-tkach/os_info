use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn no_args() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .expect("cargo_bin failed")
        .assert()
        .success()
        .stdout(all_predicate());
}

#[test]
fn all() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .expect("cargo_bin failed")
        .arg("--all")
        .assert()
        .success()
        .stdout(all_predicate());
}

#[test]
fn type_short() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .expect("cargo_bin failed")
        .arg("-t")
        .assert()
        .success()
        .stdout(type_predicate());
}

#[test]
fn type_long() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .expect("cargo_bin failed")
        .arg("--type")
        .assert()
        .success()
        .stdout(type_predicate());
}

#[test]
fn version_short() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .expect("cargo_bin failed")
        .arg("-v")
        .assert()
        .success()
        .stdout(version_predicate());
}

#[test]
fn version_long() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .expect("cargo_bin failed")
        .arg("--version")
        .assert()
        .success()
        .stdout(version_predicate());
}

#[test]
fn bitness_short() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .expect("cargo_bin failed")
        .arg("-b")
        .assert()
        .success()
        .stdout(bitness_predicate());
}

#[test]
fn bitness_long() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .expect("cargo_bin failed")
        .arg("--bitness")
        .assert()
        .success()
        .stdout(bitness_predicate());
}

fn all_predicate() -> impl Predicate<str> {
    predicate::str::starts_with("OS information:")
        .and(predicate::str::contains("Type"))
        .and(predicate::str::contains("Version"))
        .and(predicate::str::contains("Bitness"))
}

fn type_predicate() -> impl Predicate<str> {
    predicate::str::starts_with("OS type: ")
        .and(predicate::str::starts_with("OS version").not())
        .and(predicate::str::starts_with("OS bitness").not())
}

fn version_predicate() -> impl Predicate<str> {
    predicate::str::starts_with("OS version: ")
        .and(predicate::str::starts_with("OS type").not())
        .and(predicate::str::starts_with("OS bitness").not())
}

fn bitness_predicate() -> impl Predicate<str> {
    predicate::str::starts_with("OS bitness: ")
        .and(predicate::str::starts_with("OS version").not())
        .and(predicate::str::starts_with("OS type").not())
}

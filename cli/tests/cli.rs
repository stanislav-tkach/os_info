use std::path::Path;

use assert_cmd::Command;
use predicates::prelude::*;

const BIN_NAME: &str = env!("CARGO_BIN_EXE_os_info");

#[test]
fn path_is_correct() {
    assert!(Path::new(BIN_NAME).is_file());
}

#[test]
fn no_args() {
    Command::new(BIN_NAME)
        .assert()
        .success()
        .stdout(all_predicate());
}

#[test]
fn all() {
    Command::new(BIN_NAME)
        .arg("--all")
        .assert()
        .success()
        .stdout(all_predicate());
}

#[test]
fn type_short() {
    Command::new(BIN_NAME)
        .arg("-t")
        .assert()
        .success()
        .stdout(type_predicate());
}

#[test]
fn type_long() {
    Command::new(BIN_NAME)
        .arg("--type")
        .assert()
        .success()
        .stdout(type_predicate());
}

#[test]
fn version_short() {
    Command::new(BIN_NAME)
        .arg("-v")
        .assert()
        .success()
        .stdout(version_predicate());
}

#[test]
fn version_long() {
    Command::new(BIN_NAME)
        .arg("--os-version")
        .assert()
        .success()
        .stdout(version_predicate());
}

#[test]
fn bitness_short() {
    Command::new(BIN_NAME)
        .arg("-b")
        .assert()
        .success()
        .stdout(bitness_predicate());
}

#[test]
fn bitness_long() {
    Command::new(BIN_NAME)
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

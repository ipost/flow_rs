extern crate assert_cmd;
extern crate predicates;
extern crate tempdir;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs::read_to_string;
use std::process::Command;
use tempdir::TempDir;

const MINIMAL_INPUT: &str = "test;";
const MINIMAL_AST: &str = "\
all:
  process:
    step:
      expression: \"test\"\
";
const BIN_NAME: &str = "flowrs";

#[test]
fn reads_from_stdin() {
    bin().with_stdin().buffer(MINIMAL_INPUT).assert().success();
}

#[test]
fn writes_to_stdout() {
    bin()
        .with_stdin()
        .buffer(MINIMAL_INPUT)
        .assert()
        .stdout(is_dot());
}

#[test]
fn prints_ast() {
    bin()
        .args(&["-a", "-o", "/dev/null"])
        .with_stdin()
        .buffer(MINIMAL_INPUT)
        .assert()
        .stdout(predicates::str::is_match(MINIMAL_AST).unwrap());
}

#[test]
fn reads_from_file() {
    bin()
        .args(&["-f", "tests/fixtures/simple"])
        .assert()
        .success();
}

#[test]
fn writes_to_file() {
    let dir = TempDir::new("_test_").expect("Failed to create temp directory");
    let p = dir.into_path().join("foo.txt");
    bin()
        .args(&["-f", "tests/fixtures/simple"])
        .args(&["-o", p.as_os_str().to_str().unwrap()])
        .assert()
        .success();
    assert!(p.exists());
    let contents = read_to_string(p).expect("Failed to read test file");
    assert!(is_dot().eval(&contents));
}

fn is_dot() -> predicates::boolean::AndPredicate<
    predicates::str::StartsWithPredicate,
    predicates::str::EndsWithPredicate,
    str,
> {
    predicate::str::starts_with("strict digraph {").and(predicate::str::ends_with("}\n"))
}

fn bin() -> Command {
    Command::cargo_bin(BIN_NAME).unwrap()
}

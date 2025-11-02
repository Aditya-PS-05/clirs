use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_single_word() {
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd.arg("hello")
        .assert()
        .success()
        .stdout("hello\n");
}

#[test]
fn test_multiple_words() {
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd.args(["hello", "world"])
        .assert()
        .success()
        .stdout("hello world\n");
}

#[test]
fn test_multiple_words_with_spaces() {
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd.args(["hello", "world", "from", "Rust"])
        .assert()
        .success()
        .stdout("hello world from Rust\n");
}

#[test]
fn test_omit_newline_flag() {
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd.args(["-n", "hello"])
        .assert()
        .success()
        .stdout("hello");
}

#[test]
fn test_omit_newline_flag_multiple_words() {
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd.args(["-n", "hello", "world"])
        .assert()
        .success()
        .stdout("hello world");
}

#[test]
fn test_no_arguments() {
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_special_characters() {
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd.args(["hello!", "@world", "#rust"])
        .assert()
        .success()
        .stdout("hello! @world #rust\n");
}

#[test]
fn test_quoted_string() {
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd.arg("hello world")
        .assert()
        .success()
        .stdout("hello world\n");
}

#[test]
fn test_numbers() {
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd.args(["123", "456", "789"])
        .assert()
        .success()
        .stdout("123 456 789\n");
}

#[test]
fn test_mixed_content() {
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd.args(["hello", "123", "world", "456"])
        .assert()
        .success()
        .stdout("hello 123 world 456\n");
}

#[test]
fn test_empty_string() {
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd.arg("")
        .assert()
        .success()
        .stdout("\n");
}

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust Version of Echo"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("echors"));
}

use assert_cmd::{Command, assert::OutputAssertExt};
use predicates::{function::function, prelude::predicate};
use std::env;

#[cfg(test)]
fn setup() -> (String, String) {
    let key = env::var("ASPEAK_TEST_KEY").expect(
        "An azure subscription key is required to be set as ASPEAK_TEST_KEY to run the tests.",
    );
    let region = env::var("ASPEAK_TEST_REGION").expect("An azure subscription region is required to be set as ASPEAK_TEST_REGION to run the tests.");
    return (key, region);
}

fn aspeak_command() -> Command {
    let (key, region) = setup();
    let mut cmd = Command::cargo_bin("aspeak").unwrap();
    cmd.env("ASPEAK_AUTH_KEY", key)
        .arg(format!("--region={region}"));
    cmd
}

#[test]
fn cli_test_help() {
    aspeak_command()
        .arg("help")
        .unwrap()
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "A simple text-to-speech client for Azure TTS API",
        ));
}

#[test]
fn cli_test_text_simple() {
    aspeak_command()
        .arg("text")
        .arg("-o")
        .arg("-")
        .arg("hello")
        .unwrap()
        .assert()
        .success()
        .stdout(function(|v: &[u8]| !v.is_empty()));
}

#[test]
fn cli_test_text_stdin() {
    aspeak_command()
        .arg("text")
        .arg("-o")
        .arg("-")
        .write_stdin("Hello")
        .unwrap()
        .assert()
        .success()
        .stdout(function(|v: &[u8]| !v.is_empty()));
}

#[test]
fn cli_test_ssml_stdin() {
    aspeak_command()
        .arg("ssml")
        .arg("-o")
        .arg("-")
        .write_stdin("<speak version='1.0' xmlns='http://www.w3.org/2001/10/synthesis' xml:lang='en-US'><voice name='en-US-JennyNeural'>Hello, world!</voice></speak>")
        .unwrap()
        .assert()
        .success()
        .stdout(function(|v: &[u8]| !v.is_empty()));
}

#[test]
fn cli_test_ssml_invalid() {
    aspeak_command()
        .arg("ssml")
        .arg("-o")
        .arg("-")
        .write_stdin("<speak version='1.0' xmlns='http://www.w3.org/2001/10/synthesis' xml:lang='en-US'>Hello, world!</speak>")
        .assert()
        .failure()
        .stderr(function(|v: &[u8]| !v.is_empty()));
}

use assertor::{assert_that, EqualityAssertion, VecAssertion};

use crate::config;

#[test]
fn parse_cli_args_single_arg() {
    // GIVEN is a list of strings
    let args = vec!["span".to_string(), "ocean".to_string()];

    // WHEN parsing cli args
    let config = config::parse_cli_args(args);

    // THEN the config should match the args
    assert_that!(config.span).is_equal_to(true);
    assert_that!(config.query).is_equal_to("ocean".to_string());
}

#[test]
fn parse_cli_args_two_arg() {
    // GIVEN is a list of strings
    let args = vec!["sun".to_string(), "water".to_string()];

    // WHEN parsing cli args
    let config = config::parse_cli_args(args);

    // THEN the config should match the args
    assert_that!(config.span).is_equal_to(false);
    assert_that!(vec!["sun".to_string(), "water".to_string()]).contains(config.query);
}

#[test]
fn parse_cli_args_no_arg() {
    // GIVEN is a list of strings
    let args = vec![];

    // WHEN parsing cli args
    let config = config::parse_cli_args(args);

    // THEN the config should match the args
    assert_that!(config.span).is_equal_to(false);
    assert_that!(config.query).is_equal_to("wallpaper".to_string());
}

#[test]
fn parse_cli_args_only_span() {
    // GIVEN is a list of strings
    let args = vec!["span".to_string()];

    // WHEN parsing cli args
    let config = config::parse_cli_args(args);

    // THEN the config should match the args
    assert_that!(config.span).is_equal_to(true);
}

#[test]
fn is_url_true() {
    // GIVEN a url
    let url = "https://example.org";

    // WHEN checking the url
    let is_url = config::is_url(url);

    // THEN it should result into true
    assert_that!(is_url).is_equal_to(true);
}

#[test]
fn is_url_false() {
    // GIVEN a string
    let url = "hello";

    // WHEN checking the url
    let is_url = config::is_url(url);

    // THEN it should result into false
    assert_that!(is_url).is_equal_to(false);
}

#[test]
fn is_url_empty() {
    // GIVEN an empty string
    let url = "";

    // WHEN checking the url
    let is_url = config::is_url(url);

    // THEN it should result into false
    assert_that!(is_url).is_equal_to(false);
}

#[test]
fn is_local_path_true() {
    // GIVEN a local path
    let file = "/dev/null";

    // WHEN checking the file
    let is_local_path = config::is_local_path(file);

    // THEN it should result into true
    assert_that!(is_local_path).is_equal_to(true);
}

#[test]
fn is_local_path_false() {
    // GIVEN a non existing path
    let file = "/this/path/does/not/exits";

    // WHEN checking the file
    let is_local_path = config::is_local_path(file);

    // THEN it should result into false
    assert_that!(is_local_path).is_equal_to(false);
}

#[test]
fn is_local_path_empty() {
    // GIVEN a empty path
    let file = "";

    // WHEN checking the file
    let is_local_path = config::is_local_path(file);

    // THEN it should result into false
    assert_that!(is_local_path).is_equal_to(false);
}

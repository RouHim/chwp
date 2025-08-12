use assertor::{assert_that, EqualityAssertion, StringAssertion, VecAssertion};
use std::env;

use crate::config;
use crate::display::DisplayInfo;
use crate::wallhaven;

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
    assert_that!(config.query).is_equal_to("".to_string());
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

#[test]
fn parse_cli_args_with_api_key_env_var() {
    // GIVEN an API key environment variable is set
    env::set_var("WALLHAVEN_API_KEY", "test_api_key_123");
    let args = vec!["nature".to_string()];

    // WHEN parsing cli args
    let config = config::parse_cli_args(args);

    // THEN the config should include the API key
    assert_that!(config.query).is_equal_to("nature".to_string());
    assert_that!(config.api_key.is_some()).is_equal_to(true);
    assert_that!(config.api_key.unwrap()).is_equal_to("test_api_key_123".to_string());

    // Clean up
    env::remove_var("WALLHAVEN_API_KEY");
}

#[test]
fn parse_cli_args_without_api_key_env_var() {
    // GIVEN no API key environment variable is set
    env::remove_var("WALLHAVEN_API_KEY");
    let args = vec!["mountains".to_string()];

    // WHEN parsing cli args
    let config = config::parse_cli_args(args);

    // THEN the config should not include an API key
    assert_that!(config.query).is_equal_to("mountains".to_string());
    assert_that!(config.api_key.is_none()).is_equal_to(true);
}

#[test]
fn integration_test_wallhaven_url_generation() {
    use std::env;

    // Test various scenarios to ensure URL generation works correctly

    // Scenario 1: Basic single monitor, no API key
    env::remove_var("WALLHAVEN_API_KEY");
    let config1 = config::parse_cli_args(vec!["nature".to_string()]);
    let display_info1 = DisplayInfo {
        count: 1,
        resolutions: vec!["1920x1080".to_string()],
        max_single_resolution: "1920x1080".to_string(),
        total_resolution: "1920x1080".to_string(),
    };
    let url1 = wallhaven::build_request_url(&config1, &display_info1);
    assert_that!(url1).contains("q=nature");
    assert_that!(url1).contains("atleast=1920x1080");
    assert_that!(url1).contains("categories=111");
    assert_that!(url1).contains("purity=100");
    assert_that!(config1.api_key.is_none()).is_equal_to(true);

    // Scenario 2: Multi-monitor span with API key
    env::set_var("WALLHAVEN_API_KEY", "test_key_456");
    let config2 = config::parse_cli_args(vec!["span".to_string(), "landscape".to_string()]);
    let display_info2 = DisplayInfo {
        count: 2,
        resolutions: vec!["1920x1080".to_string(), "1920x1080".to_string()],
        max_single_resolution: "1920x1080".to_string(),
        total_resolution: "3840x1080".to_string(),
    };
    let url2 = wallhaven::build_request_url(&config2, &display_info2);
    assert_that!(url2).contains("q=landscape");
    assert_that!(url2).contains("atleast=3840x1080");
    assert_that!(url2).contains("apikey=test_key_456");
    assert_that!(config2.span).is_equal_to(true);
    assert_that!(config2.api_key.is_some()).is_equal_to(true);

    // Scenario 3: 4K monitor with complex query
    let config3 = config::parse_cli_args(vec!["+anime".to_string(), "-nsfw".to_string()]);
    let display_info3 = DisplayInfo {
        count: 1,
        resolutions: vec!["3840x2160".to_string()],
        max_single_resolution: "3840x2160".to_string(),
        total_resolution: "3840x2160".to_string(),
    };
    let url3 = wallhaven::build_request_url(&config3, &display_info3);
    // Should pick one of the keywords randomly
    assert_that!(url3.contains("%2Banime") || url3.contains("-nsfw")).is_equal_to(true);
    assert_that!(url3).contains("atleast=3840x2160");

    // Scenario 4: No arguments - should get random wallpapers
    let config4 = config::parse_cli_args(vec![]);
    let display_info4 = DisplayInfo {
        count: 1,
        resolutions: vec!["1920x1080".to_string()],
        max_single_resolution: "1920x1080".to_string(),
        total_resolution: "1920x1080".to_string(),
    };
    let url4 = wallhaven::build_request_url(&config4, &display_info4);
    // Should not contain q= parameter for random wallpapers
    assert_that!(url4.contains("q=")).is_equal_to(false);
    assert_that!(url4).contains("atleast=1920x1080");
    assert_that!(url4).contains("sorting=random");

    // Clean up
    env::remove_var("WALLHAVEN_API_KEY");
}

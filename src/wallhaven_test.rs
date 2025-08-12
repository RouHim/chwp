#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::display::DisplayInfo;
    use crate::wallhaven;

    #[test]
    fn test_build_request_url_basic() {
        let config = Config {
            span: false,
            query: "nature".to_string(),
            api_key: None,
        };

        let display_info = DisplayInfo {
            count: 1,
            resolutions: vec!["1920x1080".to_string()],
            max_single_resolution: "1920x1080".to_string(),
            total_resolution: "1920x1080".to_string(),
        };

        let url = wallhaven::build_request_url(&config, &display_info);

        assert!(url.contains("https://wallhaven.cc/api/v1/search"));
        assert!(url.contains("q=nature"));
        assert!(url.contains("atleast=1920x1080"));
        assert!(url.contains("categories=111"));
        assert!(url.contains("purity=100"));
        assert!(url.contains("sorting=random"));
        assert!(!url.contains("apikey="));
    }

    #[test]
    fn test_build_request_url_with_api_key() {
        let config = Config {
            span: false,
            query: "mountains".to_string(),
            api_key: Some("test_api_key".to_string()),
        };

        let display_info = DisplayInfo {
            count: 1,
            resolutions: vec!["2560x1440".to_string()],
            max_single_resolution: "2560x1440".to_string(),
            total_resolution: "2560x1440".to_string(),
        };

        let url = wallhaven::build_request_url(&config, &display_info);

        assert!(url.contains("https://wallhaven.cc/api/v1/search"));
        assert!(url.contains("q=mountains"));
        assert!(url.contains("atleast=2560x1440"));
        assert!(url.contains("apikey=test_api_key"));
    }

    #[test]
    fn test_build_request_url_span_mode() {
        let config = Config {
            span: true,
            query: "ocean".to_string(),
            api_key: None,
        };

        let display_info = DisplayInfo {
            count: 2,
            resolutions: vec!["1920x1080".to_string(), "1920x1080".to_string()],
            max_single_resolution: "1920x1080".to_string(),
            total_resolution: "3840x1080".to_string(),
        };

        let url = wallhaven::build_request_url(&config, &display_info);

        assert!(url.contains("atleast=3840x1080"));
        assert!(url.contains("categories=111"));
        assert!(url.contains("purity=100"));
        assert!(url.contains("sorting=random"));
    }

    #[test]
    fn test_url_encoding_special_characters() {
        let config = Config {
            span: false,
            query: "space & stars".to_string(),
            api_key: None,
        };

        let display_info = DisplayInfo {
            count: 1,
            resolutions: vec!["1920x1080".to_string()],
            max_single_resolution: "1920x1080".to_string(),
            total_resolution: "1920x1080".to_string(),
        };

        let url = wallhaven::build_request_url(&config, &display_info);

        assert!(url.contains("q=space%20%26%20stars"));
    }

    #[test]
    fn test_url_encoding_plus_signs() {
        let config = Config {
            span: false,
            query: "+anime +girl -nsfw".to_string(),
            api_key: None,
        };

        let display_info = DisplayInfo {
            count: 1,
            resolutions: vec!["1920x1080".to_string()],
            max_single_resolution: "1920x1080".to_string(),
            total_resolution: "1920x1080".to_string(),
        };

        let url = wallhaven::build_request_url(&config, &display_info);

        assert!(url.contains("q=%2Banime%20%2Bgirl%20-nsfw"));
    }

    #[test]
    fn test_url_encoding_unicode_characters() {
        let config = Config {
            span: false,
            query: "城市 风景".to_string(),
            api_key: None,
        };

        let display_info = DisplayInfo {
            count: 1,
            resolutions: vec!["1920x1080".to_string()],
            max_single_resolution: "1920x1080".to_string(),
            total_resolution: "1920x1080".to_string(),
        };

        let url = wallhaven::build_request_url(&config, &display_info);

        assert!(url.contains("q=%E5%9F%8E%E5%B8%82%20%E9%A3%8E%E6%99%AF"));
    }

    #[test]
    fn test_empty_query() {
        let config = Config {
            span: false,
            query: "".to_string(),
            api_key: None,
        };

        let display_info = DisplayInfo {
            count: 1,
            resolutions: vec!["1920x1080".to_string()],
            max_single_resolution: "1920x1080".to_string(),
            total_resolution: "1920x1080".to_string(),
        };

        let url = wallhaven::build_request_url(&config, &display_info);

        // Empty query should not contain q= parameter - this gets truly random wallpapers
        assert!(!url.contains("q="));
        assert!(url.contains("atleast=1920x1080"));
    }

    #[test]
    fn test_high_resolution_4k() {
        let config = Config {
            span: false,
            query: "4k wallpaper".to_string(),
            api_key: None,
        };

        let display_info = DisplayInfo {
            count: 1,
            resolutions: vec!["3840x2160".to_string()],
            max_single_resolution: "3840x2160".to_string(),
            total_resolution: "3840x2160".to_string(),
        };

        let url = wallhaven::build_request_url(&config, &display_info);

        assert!(url.contains("atleast=3840x2160"));
    }

    #[test]
    fn test_ultrawide_resolution() {
        let config = Config {
            span: false,
            query: "ultrawide".to_string(),
            api_key: None,
        };

        let display_info = DisplayInfo {
            count: 1,
            resolutions: vec!["3440x1440".to_string()],
            max_single_resolution: "3440x1440".to_string(),
            total_resolution: "3440x1440".to_string(),
        };

        let url = wallhaven::build_request_url(&config, &display_info);

        assert!(url.contains("atleast=3440x1440"));
    }

    #[test]
    fn test_multi_monitor_span() {
        let config = Config {
            span: true,
            query: "panoramic".to_string(),
            api_key: Some("api_key_123".to_string()),
        };

        let display_info = DisplayInfo {
            count: 3,
            resolutions: vec![
                "1920x1080".to_string(),
                "2560x1440".to_string(),
                "1920x1080".to_string(),
            ],
            max_single_resolution: "2560x1440".to_string(),
            total_resolution: "6400x1440".to_string(),
        };

        let url = wallhaven::build_request_url(&config, &display_info);

        assert!(url.contains("atleast=6400x1440"));
        assert!(url.contains("apikey=api_key_123"));
        assert!(url.contains("q=panoramic"));
    }

    #[test]
    fn test_api_key_with_special_characters() {
        let config = Config {
            span: false,
            query: "test".to_string(),
            api_key: Some("key-123_ABC".to_string()),
        };

        let display_info = DisplayInfo {
            count: 1,
            resolutions: vec!["1920x1080".to_string()],
            max_single_resolution: "1920x1080".to_string(),
            total_resolution: "1920x1080".to_string(),
        };

        let url = wallhaven::build_request_url(&config, &display_info);

        assert!(url.contains("apikey=key-123_ABC"));
    }

    #[test]
    fn test_wallhaven_advanced_query_syntax() {
        let config = Config {
            span: false,
            query: "@username +tag1 -tag2 type:jpg".to_string(),
            api_key: None,
        };

        let display_info = DisplayInfo {
            count: 1,
            resolutions: vec!["1920x1080".to_string()],
            max_single_resolution: "1920x1080".to_string(),
            total_resolution: "1920x1080".to_string(),
        };

        let url = wallhaven::build_request_url(&config, &display_info);

        assert!(url.contains("q=%40username%20%2Btag1%20-tag2%20type%3Ajpg"));
    }

    #[test]
    fn test_very_low_resolution() {
        let config = Config {
            span: false,
            query: "pixel art".to_string(),
            api_key: None,
        };

        let display_info = DisplayInfo {
            count: 1,
            resolutions: vec!["800x600".to_string()],
            max_single_resolution: "800x600".to_string(),
            total_resolution: "800x600".to_string(),
        };

        let url = wallhaven::build_request_url(&config, &display_info);

        assert!(url.contains("atleast=800x600"));
    }

    #[test]
    fn test_url_parameter_order() {
        let config = Config {
            span: false,
            query: "test".to_string(),
            api_key: Some("testkey".to_string()),
        };

        let display_info = DisplayInfo {
            count: 1,
            resolutions: vec!["1920x1080".to_string()],
            max_single_resolution: "1920x1080".to_string(),
            total_resolution: "1920x1080".to_string(),
        };

        let url = wallhaven::build_request_url(&config, &display_info);

        // Check that all required parameters are present
        assert!(url.starts_with("https://wallhaven.cc/api/v1/search?"));
        assert!(url.contains("q=test"));
        assert!(url.contains("atleast=1920x1080"));
        assert!(url.contains("categories=111"));
        assert!(url.contains("purity=100"));
        assert!(url.contains("sorting=random"));
        assert!(url.contains("apikey=testkey"));

        // Verify no double question marks or ampersands
        assert!(!url.contains("??"));
        assert!(!url.contains("&&"));
    }

    #[test]
    fn test_long_query_string() {
        let config = Config {
            span: false,
            query:
                "this is a very long query string with many words and special characters!@#$%^&*()"
                    .to_string(),
            api_key: None,
        };

        let display_info = DisplayInfo {
            count: 1,
            resolutions: vec!["1920x1080".to_string()],
            max_single_resolution: "1920x1080".to_string(),
            total_resolution: "1920x1080".to_string(),
        };

        let url = wallhaven::build_request_url(&config, &display_info);

        // Should be properly encoded
        assert!(url.len() > 200); // Long URL due to encoding
        assert!(url.contains("q=this%20is%20a%20very%20long"));
    }

    // JSON Response Parsing Tests
    #[test]
    fn test_parse_valid_wallhaven_response() {
        let json_response = r#"{
            "data": [
                {
                    "id": "94x38z",
                    "path": "https://w.wallhaven.cc/full/94/wallhaven-94x38z.jpg",
                    "resolution": "6742x3534"
                },
                {
                    "id": "ze1p56",
                    "path": "https://w.wallhaven.cc/full/ze/wallhaven-ze1p56.jpg",
                    "resolution": "3779x2480"
                }
            ]
        }"#;

        let result = wallhaven::parse_wallhaven_response(json_response);
        assert!(result.is_ok());

        let images = result.unwrap();
        assert_eq!(images.len(), 2);
        assert_eq!(
            images[0],
            "https://w.wallhaven.cc/full/94/wallhaven-94x38z.jpg"
        );
        assert_eq!(
            images[1],
            "https://w.wallhaven.cc/full/ze/wallhaven-ze1p56.jpg"
        );
    }

    #[test]
    fn test_parse_empty_response() {
        let json_response = r#"{
            "data": []
        }"#;

        let result = wallhaven::parse_wallhaven_response(json_response);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No wallpapers found"));
    }

    #[test]
    fn test_parse_invalid_json() {
        let invalid_json = "{ invalid json }";

        let result = wallhaven::parse_wallhaven_response(invalid_json);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to parse JSON"));
    }

    #[test]
    fn test_parse_api_error_response() {
        let json_response = r#"{
            "error": "Invalid API key provided"
        }"#;

        let result = wallhaven::parse_wallhaven_response(json_response);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Wallhaven API error: Invalid API key provided"));
    }

    #[test]
    fn test_parse_response_missing_data_field() {
        let json_response = r#"{
            "meta": {
                "current_page": 1,
                "last_page": 1
            }
        }"#;

        let result = wallhaven::parse_wallhaven_response(json_response);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No 'data' field in response"));
    }

    #[test]
    fn test_parse_response_data_not_array() {
        let json_response = r#"{
            "data": "not an array"
        }"#;

        let result = wallhaven::parse_wallhaven_response(json_response);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("'data' field is not an array"));
    }

    #[test]
    fn test_parse_response_missing_path_fields() {
        let json_response = r#"{
            "data": [
                {
                    "id": "94x38z",
                    "url": "https://wallhaven.cc/w/94x38z"
                },
                {
                    "id": "ze1p56",
                    "path": "https://w.wallhaven.cc/full/ze/wallhaven-ze1p56.jpg"
                }
            ]
        }"#;

        let result = wallhaven::parse_wallhaven_response(json_response);
        assert!(result.is_ok());

        let images = result.unwrap();
        // Should only include the one with a valid path field
        assert_eq!(images.len(), 1);
        assert_eq!(
            images[0],
            "https://w.wallhaven.cc/full/ze/wallhaven-ze1p56.jpg"
        );
    }

    #[test]
    fn test_parse_response_all_missing_path_fields() {
        let json_response = r#"{
            "data": [
                {
                    "id": "94x38z",
                    "url": "https://wallhaven.cc/w/94x38z"
                },
                {
                    "id": "ze1p56",
                    "url": "https://wallhaven.cc/w/ze1p56"
                }
            ]
        }"#;

        let result = wallhaven::parse_wallhaven_response(json_response);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("No valid wallpaper URLs found"));
    }

    #[test]
    fn test_parse_response_with_meta_data() {
        let json_response = r#"{
            "data": [
                {
                    "id": "94x38z",
                    "path": "https://w.wallhaven.cc/full/94/wallhaven-94x38z.jpg",
                    "views": 100,
                    "favorites": 5
                }
            ],
            "meta": {
                "current_page": 1,
                "last_page": 36,
                "per_page": 24,
                "total": 848
            }
        }"#;

        let result = wallhaven::parse_wallhaven_response(json_response);
        assert!(result.is_ok());

        let images = result.unwrap();
        assert_eq!(images.len(), 1);
        assert_eq!(
            images[0],
            "https://w.wallhaven.cc/full/94/wallhaven-94x38z.jpg"
        );
    }

    #[test]
    fn test_parse_large_response() {
        // Create a JSON response with many wallpapers (simulating full 24-item response)
        let mut data_items = Vec::new();
        for i in 1..=24 {
            data_items.push(format!(
                r#"{{"id": "test{:02}", "path": "https://w.wallhaven.cc/full/test/wallhaven-test{:02}.jpg"}}"#,
                i, i
            ));
        }

        let json_response = format!(r#"{{"data": [{}]}}"#, data_items.join(","));

        let result = wallhaven::parse_wallhaven_response(&json_response);
        assert!(result.is_ok());

        let images = result.unwrap();
        assert_eq!(images.len(), 24);
        assert_eq!(
            images[0],
            "https://w.wallhaven.cc/full/test/wallhaven-test01.jpg"
        );
        assert_eq!(
            images[23],
            "https://w.wallhaven.cc/full/test/wallhaven-test24.jpg"
        );
    }

    #[test]
    fn test_rate_limit_error() {
        let json_response = r#"{
            "error": "Too many requests"
        }"#;

        let result = wallhaven::parse_wallhaven_response(json_response);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Too many requests"));
    }

    #[test]
    fn test_authentication_error() {
        let json_response = r#"{
            "error": "Unauthorized"
        }"#;

        let result = wallhaven::parse_wallhaven_response(json_response);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unauthorized"));
    }

    #[test]
    fn test_no_results_error_handling() {
        // Test that we get a proper error message when no wallpapers are found
        let json_response = r#"{
            "data": []
        }"#;

        let result = wallhaven::parse_wallhaven_response(json_response);
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("No wallpapers found"));
    }

    #[test]
    fn test_empty_results_after_filtering() {
        // Test case where API returns data but no valid path fields
        let json_response = r#"{
            "data": [
                {
                    "id": "test123",
                    "url": "https://wallhaven.cc/w/test123",
                    "resolution": "1920x1080"
                },
                {
                    "id": "test456", 
                    "resolution": "2560x1440"
                }
            ]
        }"#;

        let result = wallhaven::parse_wallhaven_response(json_response);
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("No valid wallpaper URLs found"));
    }
}

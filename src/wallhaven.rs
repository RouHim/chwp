use rand::Rng;

use crate::config::Config;
use crate::display::DisplayInfo;
use crate::{display, download};

/// wallhaven api base url
const BASE_URL: &str = "https://wallhaven.cc/api/v1/search";

/// Gets an image from wallhaven
/// # Arguments
/// * `config` - The configuration to use
/// * `display_info` - The display info to use
/// # Returns
/// Result containing the image data as byte array or error message
pub fn get_image_data(config: &Config, display_info: &DisplayInfo) -> Result<Vec<u8>, String> {
    let image_url = get_image_url(config, display_info)?;
    Ok(download::get_data(&image_url))
}

/// Gets the image url from wallhaven
/// # Arguments
/// * `config` - The configuration to use
/// * `display_info` - The display info to use
/// # Returns
/// Result containing a random image url from wallhaven response or error message
fn get_image_url(config: &Config, display_info: &DisplayInfo) -> Result<String, String> {
    let request_url = build_request_url(config, display_info);
    let json_string = download::get_string(&request_url);

    let json_value: serde_json::Value =
        serde_json::from_str(json_string.as_str()).map_err(|_| "Failed to parse Wallhaven API response".to_string())?;

    // Check for API errors
    if let Some(error) = json_value.get("error") {
        return Err(format!(
            "Wallhaven API error: {}",
            error.as_str().unwrap_or("Unknown error")
        ));
    }

    let data_array = json_value
        .get("data")
        .ok_or("No 'data' field in Wallhaven response".to_string())?
        .as_array()
        .ok_or("'data' field is not an array".to_string())?;

    if data_array.is_empty() {
        return Err(format!(
            "No wallpapers found for query '{}' with the specified resolution requirements. Try a different search term or check your internet connection.",
            config.query
        ));
    }

    let images: Vec<&str> = data_array
        .iter()
        .filter_map(|item| item.get("path")?.as_str())
        .collect();

    if images.is_empty() {
        return Err("No valid wallpaper URLs found in Wallhaven response".to_string());
    }

    let random_index = rand::thread_rng().gen_range(0..images.len());
    Ok(images.get(random_index).unwrap().to_string())
}

/// Builds the request url
/// # Arguments
/// * `config` - The configuration to use
/// * `display_info` - The display info to use
/// # Returns
/// The request url as string
pub fn build_request_url(config: &Config, display_info: &DisplayInfo) -> String {
    let target_width = if config.span {
        display::get_width(&display_info.total_resolution)
    } else {
        display::get_width(&display_info.max_single_resolution)
    };

    let target_height = if config.span {
        display::get_height(&display_info.total_resolution)
    } else {
        display::get_height(&display_info.max_single_resolution)
    };

    let mut request_url = BASE_URL.to_string();
    let mut has_params = false;

    // Add query parameter only if not empty
    if !config.query.is_empty() {
        request_url.push_str("?q=");
        request_url.push_str(&urlencoding::encode(&config.query));
        has_params = true;
    }

    // Add minimum resolution requirement
    let separator = if has_params { "&" } else { "?" };
    request_url.push_str(&format!("{}atleast={}x{}", separator, target_width, target_height));

    // Set categories to all (general, anime, people)
    request_url.push_str("&categories=111");

    // Set purity to SFW only by default
    request_url.push_str("&purity=100");

    // Use random sorting for variety
    request_url.push_str("&sorting=random");

    // Add API key if provided
    if let Some(api_key) = &config.api_key {
        request_url.push_str("&apikey=");
        request_url.push_str(api_key);
    }

    request_url
}

/// Parses a Wallhaven API response JSON string and extracts image URLs
/// # Arguments
/// * `json_response` - The JSON response string from the API
/// # Returns
/// A Result containing either a vector of image URLs or an error message
#[cfg(test)]
pub fn parse_wallhaven_response(json_response: &str) -> Result<Vec<String>, String> {
    let json_value: serde_json::Value = serde_json::from_str(json_response)
        .map_err(|_| "Failed to parse JSON response".to_string())?;

    // Check for API errors
    if let Some(error) = json_value.get("error") {
        return Err(format!(
            "Wallhaven API error: {}",
            error.as_str().unwrap_or("Unknown error")
        ));
    }

    let data_array = json_value
        .get("data")
        .ok_or("No 'data' field in response".to_string())?
        .as_array()
        .ok_or("'data' field is not an array".to_string())?;

    if data_array.is_empty() {
        return Err("No wallpapers found in response".to_string());
    }

    let images: Vec<String> = data_array
        .iter()
        .filter_map(|item| item.get("path")?.as_str().map(|s| s.to_string()))
        .collect();

    if images.is_empty() {
        return Err("No valid wallpaper URLs found in response".to_string());
    }

    Ok(images)
}

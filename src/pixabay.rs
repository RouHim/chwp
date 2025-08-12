use rand::Rng;

use crate::config::Config;
use crate::display::DisplayInfo;
use crate::{display, download};

/// pixabay api base url
const BASE_URL: &str =
    "https://pixabay.com/api/?key=15495421-a5108e860086b11eddaea0efa&per_page=200";

/// Gets an image from pixabay
/// # Arguments
/// * `config` - The configuration to use
/// * `display_info` - The display info to use
/// # Returns
/// The image data as byte array
pub fn get_image_data(config: &Config, display_info: &DisplayInfo) -> Vec<u8> {
    let image_url = get_image_url(config, display_info);
    download::get_data(&image_url)
}

/// Gets the image url from pixabay
/// # Arguments
/// * `config` - The configuration to use
/// * `display_info` - The display info to use
/// # Returns
/// A random image url from pixabay response
fn get_image_url(config: &Config, display_info: &DisplayInfo) -> String {
    let request_url = build_request_url(config, display_info);
    let json_string = download::get_string(&request_url);

    let json_value: serde_json::Value = serde_json::from_str(json_string.as_str()).unwrap();

    let images: Vec<&str> = json_value
        .get("hits")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|hit| hit.get("imageURL").unwrap().as_str().unwrap())
        .collect();

    let random_index = rand::thread_rng().gen_range(0..images.len());
    images.get(random_index).unwrap().to_string()
}

/// Builds the request url
/// # Arguments
/// * `config` - The configuration to use
/// * `display_info` - The display info to use
/// # Returns
/// The request url as string
fn build_request_url(config: &Config, display_info: &DisplayInfo) -> String {
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
    // Append query param
    request_url.push_str("&q=");
    request_url.push_str(&config.query);

    // Append width param
    request_url.push_str("&min_width=");
    request_url.push_str(&target_width);

    // Append height param
    request_url.push_str("&min_height=");
    request_url.push_str(&target_height);

    request_url
}

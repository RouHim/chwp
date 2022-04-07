use rand::Rng;

use crate::config::Config;
use crate::display::DisplayInfo;
use crate::file_receiver;

const BASE_URL: &str = "https://pixabay.com/api/?key=15495421-a5108e860086b11eddaea0efa&per_page=25";

pub fn get_random_image(config: &Config, display_info: &DisplayInfo) -> Vec<u8> {
    let image_url = build_image_url(config, display_info);
    file_receiver::download_data(&image_url)
}

fn build_image_url(config: &Config, display_info: &DisplayInfo) -> String {
    let request_url = build_request_url(config, display_info);
    let json_string = download_as_string(&request_url);

    let value: serde_json::Value = serde_json::from_str(json_string.as_str()).unwrap();
    let images: Vec<&str> = value
        .get("hits").unwrap()
        .as_array().unwrap()
        .iter()
        .map(|hit| hit.get("imageURL").unwrap().as_str().unwrap())
        .collect();

    let random_index = rand::thread_rng().gen_range(0..images.len());
    return images.get(random_index).unwrap().to_string();
}

fn download_as_string(request_url: &String) -> String {
    let data = file_receiver::download_data(request_url);
    String::from_utf8(data).unwrap()
}

fn build_request_url(config: &Config, display_info: &DisplayInfo) -> String {
    let mut target_width = get_width(&display_info.max_single_resolution);
    if config.span {
        target_width = get_width(&display_info.total_resolution);
    }

    let mut target_height = get_height(&display_info.max_single_resolution);
    if config.span {
        target_height = get_height(&display_info.total_resolution);
    }

    let mut request_url = BASE_URL.to_string();
    append_str(&mut request_url, "&q=", &config.keyword);
    append_str(&mut request_url, "&min_width=", target_width);
    append_str(&mut request_url, "&min_height=", target_height);

    request_url
}

fn append_str(base_string: &mut String, str1: &str, str2: &str) {
    base_string.push_str(&[str1, str2].join(""))
}

fn get_width(resolution_string: &str) -> &str {
    return resolution_string.split('x')
        .next()
        .expect("wrong display resolution format");
}

fn get_height(resolution_string: &str) -> &str {
    return resolution_string.split('x')
        .nth(1)
        .expect("wrong display resolution format");
}

use curl::easy::Easy;

use rand::Rng;

use crate::config::Config;
use crate::display::DisplayInfo;

const BASE_URL: &str = "https://pixabay.com/api/?key=15495421-a5108e860086b11eddaea0efa&per_page=50";

pub fn get_image_url(config: Config, display_info: DisplayInfo) -> String {
    let request_url = build_request_url(config, display_info);
    let json_string = get_data_from_url(&request_url);

    let value: serde_json::Value = serde_json::from_str(json_string.as_str()).unwrap();
    let hits = value
        .get("hits")
        .unwrap()
        .as_array()
        .unwrap();

    let mut images = Vec::new();
    for entry in hits {
        let image_url = entry.get("imageURL").unwrap().as_str().unwrap();
        images.push(image_url);
    }

    let random_index = rand::thread_rng().gen_range(0, images.len() - 1);

    return images.get(random_index).unwrap().to_string();
}

fn get_data_from_url(request_url: &String) -> String {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(request_url.as_str()).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    return String::from_utf8(data).expect("json data parsing");
}

fn build_request_url(config: Config, display_info: DisplayInfo) -> String {
    let mut target_width = get_width(&display_info.max_single_resolution);
    if config.span {
        target_width = get_width(&display_info.total_resolution);
    }

    let mut target_height = get_height(&display_info.max_single_resolution);
    if config.span {
        target_height = get_height(&display_info.total_resolution);
    }

    let mut request_url = BASE_URL.to_string();
    append_str(&mut request_url, "&q=", config.keywords.get(0).unwrap());
    append_str(&mut request_url, "&min_width=", &target_width);
    append_str(&mut request_url, "&min_height=", &target_height);

    return request_url;
}

fn append_str(base_string: &mut String, str1: &str, str2: &String) {
    base_string.push_str(&[str1, str2.as_str()].join(""))
}

fn get_width(resolution_string: &String) -> String {
    return resolution_string.split("x")
        .next()
        .expect("wrong display resolution format")
        .to_string();
}

fn get_height(resolution_string: &String) -> String {
    return resolution_string.split("x")
        .skip(1)
        .next()
        .expect("wrong display resolution format")
        .to_string();
}

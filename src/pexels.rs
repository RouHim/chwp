use rand::Rng;

use crate::config::Config;
use crate::display::DisplayInfo;
use crate::file_receiver;


pub fn get_random_image(config: &Config, display_info: &DisplayInfo) -> Vec<u8> {
    let image_url = build_image_url(config, display_info);
    println!("{}", image_url);
    file_receiver::download_data(&image_url)
}

fn build_image_url(config: &Config, display_info: &DisplayInfo) -> String {
    if config.span {
        format!("https://source.unsplash.com/{}/?{}", display_info.total_resolution, config.keyword)
    } else {
        format!("https://source.unsplash.com/{}/?{}", display_info.max_single_resolution, config.keyword)
    }
}
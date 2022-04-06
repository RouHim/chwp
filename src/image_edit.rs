use std::io::BufWriter;
use std::time::Instant;

use image::{GenericImageView, ImageFormat};

use crate::display::DisplayInfo;

pub fn scale_image(image_data: &[u8], span: bool, display_info: &DisplayInfo) -> Vec<u8> {
    println!("{} bytes", image_data.len());

    let read_s = Instant::now();
    let mut img = image::load_from_memory(image_data).unwrap();
    // let mut img = image::load_from_memory(&image_data).unwrap();
    println!("read time: {:.2?}", read_s.elapsed());

    let display_ratio = calculate_display_ratio(span, &display_info);
    let img_width = img.width();
    let img_height = img.height();
    let img_ratio = img_width as f32 / img_height as f32;

    let new_image_width;
    let new_image_height;
    let mut x_start = 0;
    let mut y_start = 0;

    if img_ratio <= display_ratio {
        new_image_width = img_width;
        new_image_height = (img_width as f32 / display_ratio) as u32;

        y_start = (img_height / 2) - (new_image_height / 2);
    } else {
        new_image_width = (img_height as f32 * display_ratio) as u32;
        new_image_height = img_height;

        x_start = (img_width / 2) - (new_image_width / 2);
    }

    println!("Image {}x{}", img_width, img_height);
    println!("New image {}x{}", new_image_width, new_image_height);

    let crop_s = Instant::now();
    img = img.crop(x_start, y_start, new_image_width, new_image_height);
    println!("crop time: {:.2?}", crop_s.elapsed());

    let write_s = Instant::now();
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut bytes, ImageFormat::Jpeg)
        .expect("Unable to write data");
    println!("write time: {:.2?}", write_s.elapsed());

    bytes
}

fn calculate_display_ratio(span: bool, display_info: &&DisplayInfo) -> f32 {
    let mut display_width = get_width(&display_info.max_single_resolution);
    let mut display_height = get_height(&display_info.max_single_resolution);
    if span { display_width = get_width(&display_info.total_resolution); }
    if span { display_height = get_height(&display_info.total_resolution); }
    println!("target display reso: {}x{}", display_width, display_height);
    display_width as f32 / display_height as f32
}

fn get_width(resolution_string: &str) -> usize {
    return resolution_string.split('x')
        .next()
        .expect("wrong display resolution format")
        .parse()
        .unwrap();
}

fn get_height(resolution_string: &str) -> usize {
    return resolution_string.split('x')
        .nth(1)
        .expect("wrong display resolution format")
        .parse().unwrap();
}
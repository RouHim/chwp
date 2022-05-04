use image::DynamicImage;

use crate::display::DisplayInfo;

pub fn scale_image(image_data: Vec<u8>, span: bool, display_info: &DisplayInfo) -> DynamicImage {
    let mut img = image::load_from_memory(&image_data).unwrap();

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

    img.crop(x_start, y_start, new_image_width, new_image_height)
}

fn calculate_display_ratio(span: bool, display_info: &&DisplayInfo) -> f32 {
    let mut display_width = get_width(&display_info.max_single_resolution);
    let mut display_height = get_height(&display_info.max_single_resolution);
    if span {
        display_width = get_width(&display_info.total_resolution);
    }
    if span {
        display_height = get_height(&display_info.total_resolution);
    }
    display_width as f32 / display_height as f32
}

fn get_width(resolution_string: &str) -> usize {
    return resolution_string
        .split('x')
        .next()
        .expect("wrong display resolution format")
        .parse()
        .unwrap();
}

fn get_height(resolution_string: &str) -> usize {
    return resolution_string
        .split('x')
        .nth(1)
        .expect("wrong display resolution format")
        .parse()
        .unwrap();
}

use image::DynamicImage;

use crate::display::DisplayInfo;

/// Scale the image to fit the display
/// # Arguments
/// * `image_data` - The image data to scale
/// * `span` - Whether to span the image or not
/// * `display_info` - The display info to use
/// # Returns the scaled image
/// # Example
/// ```
/// use image_edit::scale_image;
/// use display::DisplayInfo;
///     let image_data = image::open("test.png").unwrap();
///     let display_info = DisplayInfo {
///     width: 1920,
///     height: 1080,
///    };
///    let scaled_image = scale_image(image_data, true, &display_info);
///     assert_eq!(scaled_image.width(), 1920);
///    assert_eq!(scaled_image.height(), 1080);
///     assert_eq!(scaled_image.format(), image::ImageFormat::PNG);
/// ```
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

/// Calculate the display resolution ratio
/// # Arguments
/// * `span` - Whether to span the image or not
/// * `display_info` - The display info to use
/// # Returns the display resolution ratio
/// # Example
/// ```
/// use image_edit::calculate_display_ratio;
/// use display::DisplayInfo;
///    let display_info = DisplayInfo {
///    width: 1920,
///   height: 1080,
/// };
///   let display_ratio = calculate_display_ratio(true, &display_info);
///  assert_eq!(display_ratio, 1.7777777777777777);
/// ```
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

/// Gets the width of the resolution string
/// # Arguments
/// * `resolution_string` - The resolution to get the width of
/// # Returns the width of the resolution
/// # Example
/// ```
/// use image_edit::get_width;
/// use display::DisplayInfo;
///   let display_info = DisplayInfo {
///  width: 1920,
/// height: 1080,
/// };
///  let display_width = get_width(&display_info.max_single_resolution);
/// assert_eq!(display_width, 1920);
/// ```
fn get_width(resolution_string: &str) -> usize {
    return resolution_string
        .split('x')
        .next()
        .expect("wrong display resolution format")
        .parse()
        .unwrap();
}

/// Gets the height of the resolution string
/// # Arguments
/// * `resolution` - The resolution to get the height of
/// # Returns the height of the resolution
/// # Example
/// ```
/// use image_edit::get_height;
/// use display::DisplayInfo;
///  let display_info = DisplayInfo {
/// width: 1920,
/// height: 1080,
/// };
/// let display_height = get_height(&display_info.max_single_resolution);
/// assert_eq!(display_height, 1080);
/// ```
fn get_height(resolution_string: &str) -> usize {
    return resolution_string
        .split('x')
        .nth(1)
        .expect("wrong display resolution format")
        .parse()
        .unwrap();
}

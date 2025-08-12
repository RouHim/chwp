use std::path::{Path, PathBuf};
use std::{env, fs};

use assertor::{assert_that, EqualityAssertion};
use rand::Rng;

use crate::display::DisplayInfo;
use crate::{download, image_processor};

const TEST_IMAGE_URL: &str = "https://www.w3.org/comm/assets/icons/megaphone.png";
const TEST_FOLDER_NAME: &str = "image_processor_test";

#[test]
fn scale_image_same_aspect_ratio_but_smaller_screen() {
    // GIVEN is a image file of size 350x350
    // AND a display with size the of 35x35
    let test_dir = create_test_dir();
    let test_image = create_test_image(TEST_IMAGE_URL, &test_dir, "test.jpg");
    let display_info = DisplayInfo {
        count: 1,
        resolutions: vec!["35x35".to_string()],
        total_resolution: "35x35".to_string(),
        max_single_resolution: "35x35".to_string(),
    };

    // WHEN scaling the image to fit the display
    let scaled_image = image_processor::scale_image(test_image.1, false, &display_info);

    // THEN the image should be scaled to fit the display (keeps aspect ration but does not scales down)
    assert_that!(scaled_image.width()).is_equal_to(350);
    assert_that!(scaled_image.height()).is_equal_to(350);

    // cleanup
    cleanup(&test_dir);
}

#[test]
fn scale_image_different_aspect_ratio_but_smaller_screen() {
    // GIVEN is a image file of size 350x350
    // AND a display with size the of 50x25 (2:1)
    let test_dir = create_test_dir();
    let test_image = create_test_image(TEST_IMAGE_URL, &test_dir, "test.jpg");
    let display_info = DisplayInfo {
        count: 1,
        resolutions: vec!["50x25".to_string()],
        total_resolution: "50x25".to_string(),
        max_single_resolution: "50x25".to_string(),
    };

    // WHEN scaling the image to fit the display
    let scaled_image = image_processor::scale_image(test_image.1, false, &display_info);

    // THEN the image should be scaled to fit the display (keeps aspect ration but does not scales down 2:1)
    assert_that!(scaled_image.width()).is_equal_to(350);
    assert_that!(scaled_image.height()).is_equal_to(175);

    // cleanup
    cleanup(&test_dir);
}

#[test]
fn scale_image_two_displays_without_span() {
    // GIVEN is a image file of size 350x350
    // AND a two displays with size the of 50x25 and 50x25 (both 2:1)
    let test_dir = create_test_dir();
    let test_image = create_test_image(TEST_IMAGE_URL, &test_dir, "test.jpg");
    let display_info = DisplayInfo {
        count: 2,
        resolutions: vec!["50x25".to_string(), "50x25".to_string()],
        total_resolution: "100x25".to_string(),
        max_single_resolution: "50x25".to_string(),
    };

    // WHEN scaling the image to fit the display without spanning multiple displays
    let scaled_image = image_processor::scale_image(test_image.1, false, &display_info);

    // THEN the image should not be spanned across multiple displays
    assert_that!(scaled_image.width()).is_equal_to(350);
    assert_that!(scaled_image.height()).is_equal_to(175);

    // cleanup
    cleanup(&test_dir);
}

#[test]
fn scale_image_two_displays_with_span() {
    // GIVEN is a image file of size 350x350
    // AND a two displays with size the of 50x25 and 50x25 (both 2:1, total ratio is 4:1)
    let test_dir = create_test_dir();
    let test_image = create_test_image(TEST_IMAGE_URL, &test_dir, "test.jpg");
    let display_info = DisplayInfo {
        count: 2,
        resolutions: vec!["50x25".to_string(), "50x25".to_string()],
        total_resolution: "100x25".to_string(),
        max_single_resolution: "50x25".to_string(),
    };

    // WHEN scaling the image to fit the display with spanning multiple displays
    let scaled_image = image_processor::scale_image(test_image.1, true, &display_info);

    // THEN the image should be spanned across multiple displays (ratio: 4:1)
    assert_that!(scaled_image.width()).is_equal_to(350);
    assert_that!(scaled_image.height()).is_equal_to(87);

    // cleanup
    cleanup(&test_dir);
}

/// Cleans up the test folder.
fn cleanup(test_dir: &Path) {
    let _ = fs::remove_dir_all(test_dir);
}

/// Downloads a test image into the provided directory.
/// # Returns the path to the temporary test image and the data of the test image.
fn create_test_image(url: &str, test_dir: &Path, image_name: &str) -> (String, Vec<u8>) {
    let test_image_path = test_dir.join(image_name);

    let image_data = download::get_data(url);
    fs::write(&test_image_path, &image_data).unwrap_or_else(|_| {
        panic!(
            "could not write data to file {}",
            test_image_path.to_str().unwrap()
        )
    });

    (test_image_path.to_str().unwrap().to_string(), image_data)
}

/// Creates a temporary folder
/// # Returns the path to the temporary folder.
fn create_test_dir() -> PathBuf {
    let random_string = rand::rng().random::<u32>().to_string();
    let test_dir: PathBuf = env::temp_dir().join(TEST_FOLDER_NAME).join(random_string);

    if test_dir.exists() {
        fs::remove_dir_all(&test_dir).expect("Failed to remove test dir");
    }

    fs::create_dir_all(&test_dir).unwrap();
    test_dir
}

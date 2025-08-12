use std::path::{Path, PathBuf};
use std::{env, fs};

use assertor::{assert_that, EqualityAssertion, VecAssertion};
use rand::Rng;

use crate::{download, filesystem};

const TEST_JPEG_URL: &str = "https://www.w3.org/People/mimasa/test/imgformat/img/w3c_home.jpg";
const TEST_FOLDER_NAME: &str = "filesystem_test";

#[test]
fn read_file_single_element() {
    // GIVEN is a image file
    let test_dir = create_test_dir();
    let test_image = create_test_image(TEST_JPEG_URL, &test_dir, "test.jpg");

    // WHEN reading the file
    let image_data = filesystem::read_file(&test_image.0);

    // THEN the image data should be correct
    assert_that!(image_data).is_equal_to(test_image.1);

    // cleanup
    cleanup(&test_dir);
}

#[test]
fn read_file_two_element() {
    // GIVEN is are two image files
    let test_dir = create_test_dir();
    let test_image = create_test_image(TEST_JPEG_URL, &test_dir, "test.jpg");
    let test_image2 = create_test_image(TEST_JPEG_URL, &test_dir, "test2.jpg");

    // WHEN reading the whole directory
    let image_data = filesystem::read_file(test_dir.as_path().to_str().unwrap());

    // THEN a random image of the directory should be returned
    assert_that!(vec![test_image.1, test_image2.1]).contains(image_data);

    // cleanup
    cleanup(&test_dir);
}

#[test]
fn read_file_text_file() {
    // GIVEN is a text file
    let test_dir = create_test_dir();
    let test_image = create_test_image(TEST_JPEG_URL, &test_dir, "test.txt");

    // WHEN reading the file
    let image_data = filesystem::read_file(&test_image.0);

    // THEN the file should be read
    assert_that!(image_data).is_equal_to(test_image.1);

    // cleanup
    cleanup(&test_dir);
}

#[test]
fn read_file_text_file_whole_dir() {
    // GIVEN is a text file
    let test_dir = create_test_dir();
    create_test_image(TEST_JPEG_URL, &test_dir, "test.txt");

    // WHEN reading the whole directory
    let image_data = filesystem::read_file(test_dir.as_path().to_str().unwrap());

    // THEN no an empty vector should be returned
    assert_that!(image_data).is_equal_to(vec![]);

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

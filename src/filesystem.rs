use std::fs;
use std::fs::metadata;
use std::path::PathBuf;

use rand::Rng;

/// Reads data from a local file path
/// If the provided path is a directory, a random image is chosen
pub fn read_file(file_path: &str) -> Vec<u8> {
    if metadata(file_path).unwrap().is_file() {
        fs::read(file_path).expect("Unable to read file")
    } else {
        read_random_file_from_directory(file_path)
    }
}

/// Reads a random image from a directory
/// If the directory is empty, returns an empty vector
/// If the directory is not empty, returns a random image
fn read_random_file_from_directory(directory_path: &str) -> Vec<u8> {
    let paths = fs::read_dir(directory_path).unwrap();

    let mut images = vec![];

    for path in paths {
        let dir_entry = path.unwrap();
        if dir_entry.metadata().unwrap().is_file() && is_picture(dir_entry.path()) {
            images.push(dir_entry.path().to_str().unwrap().to_string())
        }
    }

    if images.is_empty() {
        return vec![];
    }

    let random_index = rand::thread_rng().gen_range(0..images.len());
    return read_file(images.get(random_index).unwrap());
}

/// Check if a file is an image
/// Allowed file extension are: jpg, jpeg, png, bmp, gif, tiff, webp
fn is_picture(file_path: PathBuf) -> bool {
    let file_extension = file_path
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
        .to_lowercase();

    file_extension == "jpg"
        || file_extension == "jpeg"
        || file_extension == "png"
        || file_extension == "bmp"
        || file_extension == "gif"
        || file_extension == "tiff"
        || file_extension == "webp"
}

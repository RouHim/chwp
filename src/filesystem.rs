use std::fs;
use std::fs::metadata;
use std::path::PathBuf;

use rand::Rng;

/// Expand a leading '~/' to the user's home directory
fn expand_tilde(path: &str) -> String {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest).to_string_lossy().into_owned();
        }
    }
    path.to_string()
}
use crate::utils::expand_tilde;
/// Reads data from a local file path
/// If the provided path is a directory, a random image is chosen
pub fn read_file(file_path: &str) -> Vec<u8> {
    let file_path = expand_tilde(file_path);
    if metadata(&file_path).unwrap().is_file() {
        fs::read(&file_path).expect("Unable to read file")
    } else {
        read_random_file_from_directory(&file_path)
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
    read_file(images.get(random_index).unwrap())
}

/// Check if a file is an image
/// Allowed file extension are: jpg, jpeg, png, bmp, gif, tiff, webp
fn is_picture(file_path: PathBuf) -> bool {
    let Some(ext) = file_path.extension().and_then(|e| e.to_str()) else {
        return false;
    };
    let file_extension = ext.to_lowercase();

    file_extension == "jpg"
        || file_extension == "jpeg"
        || file_extension == "png"
        || file_extension == "bmp"
        || file_extension == "gif"
        || file_extension == "tiff"
        || file_extension == "webp"
}

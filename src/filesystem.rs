use std::fs;
use std::fs::metadata;
use std::path::PathBuf;

use rand::Rng;

use crate::utils::expand_tilde;
/// Reads data from a local file path
/// If the provided path is a directory, a random image is chosen
pub fn read_file(file_path: &str) -> Vec<u8> {
    let file_path = expand_tilde(file_path);
    if let Ok(meta) = metadata(&file_path) {
        if meta.is_file() {
            return fs::read(&file_path).expect("Unable to read file");
        }
    }
    read_random_file_from_directory(&file_path)
}

/// Reads a random image from a directory
/// If the directory is empty, returns an empty vector
/// If the directory is not empty, returns a random image
fn read_random_file_from_directory(directory_path: &str) -> Vec<u8> {
    let mut images = vec![];

    if let Ok(paths) = fs::read_dir(directory_path) {
        for dir_entry in paths.flatten() {
            if let Ok(meta) = dir_entry.metadata() {
                if meta.is_file() && is_picture(dir_entry.path()) {
                    if let Some(p) = dir_entry.path().to_str() {
                        images.push(p.to_string());
                    }
                }
            }
        }
    }

    if images.is_empty() {
        return vec![];
    }

    let random_index = rand::rng().random_range(0..images.len());
    // Safe due to non-empty check above
    read_file(&images[random_index])
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

use std::env;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use image::DynamicImage;

use crate::cli::execute_command;
use crate::config::Config;
use crate::{gnome, kde, xfce};

/// Changes the wallpaper to the given image
/// Respective display managers are called to change the wallpaper
/// # Arguments
/// * `image_data` - The image to set as the wallpaper
/// * `config` - The application configuration
pub fn change_wallpaper(image_data: DynamicImage, config: &Config) {
    let display_manager = get_display_manager();

    let picture_option = match config.span {
        true => "spanned".to_string(),
        false => "scaled".to_string(),
    };

    clear_wallpaper_dir();

    let wallpaper_file_path = persist_to_file(image_data);
    let wallpaper_file_path_fqdn = format!("file://{wallpaper_file_path}");

    if display_manager.contains("gnome") {
        set_gnome_wallpaper(&picture_option, &wallpaper_file_path_fqdn);
    } else if display_manager.contains("cinnamon") {
        set_cinnamon_wallpaper(&picture_option, &wallpaper_file_path_fqdn);
    } else if display_manager.contains("deepin") {
        set_deepin_wallpaper(&picture_option, &wallpaper_file_path_fqdn);
    } else if display_manager.contains("plasma") || display_manager.contains("kde") {
        kde::set_wallpaper(&wallpaper_file_path);
        kde::set_lockscreen(&wallpaper_file_path);
    } else if display_manager.contains("xfce") {
        set_xfce_wallpaper(&wallpaper_file_path);
    } else {
        println!("{} is not supported yet.", display_manager)
    }
}

/// Sets the wallpaper for xfce
/// If `feh` is installed use it to set the wallpaper
fn set_xfce_wallpaper(wallpaper_file_path: &str) {
    let is_feh_installed = Path::new("/bin/feh").exists() || Path::new("/usr/bin/feh").exists();

    if is_feh_installed {
        execute_command(format!("feh --bg-scale {wallpaper_file_path}").as_str());
    } else {
        xfce::set_wallpaper(wallpaper_file_path);
    }
}

/// Sets the wallpaper and lockscreen for deepin
fn set_deepin_wallpaper(picture_option: &str, wallpaper_file_path_fqdn: &str) {
    gnome::write_settings(
        "com.deepin.wrap.gnome.desktop.background picture-uri",
        wallpaper_file_path_fqdn,
    );
    gnome::write_settings(
        "com.deepin.wrap.gnome.desktop.background picture-options",
        picture_option,
    );
    gnome::write_settings(
        "com.deepin.wrap.gnome.desktop.screensaver picture-uri",
        wallpaper_file_path_fqdn,
    );
    gnome::write_settings(
        "com.deepin.wrap.gnome.desktop.screensaver picture-options",
        picture_option,
    );
}

/// Sets the wallpaper and lockscreen for cinnamon
fn set_cinnamon_wallpaper(picture_option: &str, wallpaper_file_path_fqdn: &str) {
    gnome::write_settings(
        "org.cinnamon.desktop.background picture-uri",
        wallpaper_file_path_fqdn,
    );
    gnome::write_settings(
        "org.cinnamon.desktop.background picture-options",
        picture_option,
    );
}

fn set_gnome_wallpaper(picture_option: &str, wallpaper_file_path_fqdn: &str) {
    gnome::write_settings(
        "org.gnome.desktop.background picture-uri",
        wallpaper_file_path_fqdn,
    );
    gnome::write_settings(
        "org.gnome.desktop.background picture-uri-dark",
        wallpaper_file_path_fqdn,
    );
    gnome::write_settings(
        "org.gnome.desktop.background picture-options",
        picture_option,
    );
    gnome::write_settings(
        "org.gnome.desktop.screensaver picture-uri",
        wallpaper_file_path_fqdn,
    );
    gnome::write_settings(
        "org.gnome.desktop.screensaver picture-uri-dark",
        wallpaper_file_path_fqdn,
    );
    gnome::write_settings(
        "org.gnome.desktop.screensaver picture-options",
        picture_option,
    );
}

/// Gets the name of the current display manager
fn get_display_manager() -> String {
    return env::var("XDG_CURRENT_DESKTOP")
        .unwrap()
        .trim()
        .to_lowercase();
}

/// Clears the wallpaper directory
fn clear_wallpaper_dir() {
    let wallpaper_dir = dirs::home_dir().unwrap().join(".cache").join("chwp");
    let _ = std::fs::remove_dir_all(&wallpaper_dir);
    std::fs::create_dir_all(&wallpaper_dir).expect("wallpaper path creation failed");
}

/// Stores the image to a file and returns the path to the file
/// # Arguments
/// * `image` - The image to store
/// # Returns the path to the file
fn persist_to_file(image_data: DynamicImage) -> String {
    let path = build_target_path();
    image_data
        .save(path.as_str())
        .expect("Unable to save image");
    path
}

/// Builds the path to the wallpaper file
/// The path is based on the current time
fn build_target_path() -> String {
    let current_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();
    let file_name = format!("{current_millis}.jpg");

    dirs::home_dir()
        .unwrap()
        .join(".cache")
        .join("chwp")
        .join(file_name)
        .into_os_string()
        .into_string()
        .unwrap()
}

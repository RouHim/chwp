use std::env;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use image::DynamicImage;

use crate::cli::execute_command;
use crate::config::Config;
use crate::{gnome, kde, xfce};

pub fn change_wallpaper(image_data: DynamicImage, config: &Config) {
    let display_manager = get_display_manager();

    let picture_option = match config.span {
        true => "spanned".to_string(),
        false => "scaled".to_string(),
    };

    clear_wallpaper_dir();
    let wallpaper_file_path = persist_to_file(image_data);
    let wallpaper_file_path_fqdn = &["file://", wallpaper_file_path.as_str()].join("");

    if display_manager.contains("gnome") {
        gnome::write_settings(
            &"org.gnome.desktop.background picture-uri".to_string(),
            wallpaper_file_path_fqdn,
        );
        gnome::write_settings(
            &"org.gnome.desktop.background picture-options".to_string(),
            &picture_option,
        );
        gnome::write_settings(
            &"org.gnome.desktop.screensaver picture-uri".to_string(),
            wallpaper_file_path_fqdn,
        );
        gnome::write_settings(
            &"org.gnome.desktop.screensaver picture-options".to_string(),
            &picture_option,
        );
    } else if display_manager.contains("cinnamon") {
        gnome::write_settings(
            &"org.cinnamon.desktop.background picture-uri".to_string(),
            wallpaper_file_path_fqdn,
        );
        gnome::write_settings(
            &"org.cinnamon.desktop.background picture-options".to_string(),
            &picture_option,
        );
    } else if display_manager.contains("deepin") {
        gnome::write_settings(
            &"com.deepin.wrap.gnome.desktop.background picture-uri".to_string(),
            wallpaper_file_path_fqdn,
        );
        gnome::write_settings(
            &"com.deepin.wrap.gnome.desktop.background picture-options".to_string(),
            &picture_option,
        );
        gnome::write_settings(
            &"com.deepin.wrap.gnome.desktop.screensaver picture-uri".to_string(),
            wallpaper_file_path_fqdn,
        );
        gnome::write_settings(
            &"com.deepin.wrap.gnome.desktop.screensaver picture-options".to_string(),
            &picture_option,
        );
    } else if display_manager.contains("plasma") || display_manager.contains("kde") {
        kde::set_wallpaper(&wallpaper_file_path);
        kde::set_lockscreen(&wallpaper_file_path);
    } else if display_manager.contains("xfce") {
        if Path::new("/bin/feh").exists() || Path::new("/usr/bin/feh").exists() {
            execute_command(&["feh --bg-scale ", &wallpaper_file_path].join(""));
        } else {
            xfce::set_wallpaper(&wallpaper_file_path);
        }
    } else {
        println!("{} is not supported yet.", display_manager)
    }
}

fn get_display_manager() -> String {
    return env::var("XDG_CURRENT_DESKTOP")
        .unwrap()
        .trim()
        .to_lowercase();
}

fn clear_wallpaper_dir() {
    let path: PathBuf = [dirs::home_dir().unwrap().to_str().unwrap(), ".wallpaper"]
        .iter()
        .collect();
    std::fs::remove_dir_all(&path).expect("wallpaper cleanup failed");
    std::fs::create_dir_all(&path).expect("wallpaper path creation failed");
}

fn persist_to_file(image_data: DynamicImage) -> String {
    let path = build_target_path();
    image_data
        .save(path.as_str())
        .expect("Unable to save image");
    path
}

fn build_target_path() -> String {
    let mut user_home = dirs::home_dir().unwrap();
    let random_file_name = [
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string(),
        ".jpg".to_string(),
    ]
    .join("");
    user_home.push(".wallpaper");
    user_home.push(random_file_name);
    user_home.into_os_string().into_string().unwrap()
}

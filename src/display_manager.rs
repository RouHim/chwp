use std::env;

use crate::config::Config;
use crate::gnome;

pub fn change_wallpaper(image_data: &Vec<u8>, config: &Config) {
    let display_manager = get_display_manager();

    let picture_option = match config.span {
        true => "spanned".to_string(),
        false => "scaled".to_string()
    };

    //TODO:
    let wallpaper_path: String = "".to_string();
    let file_wallpaper_path = ["file://".to_string(), wallpaper_path].join("");


    if display_manager.contains("gnome") {
        gnome::write_settings(&"org.gnome.desktop.background picture-uri".to_string(), file_wallpaper_path);
        gnome::write_settings(&"org.gnome.desktop.background picture-options".to_string(), &picture_option);
        gnome::write_settings(&"org.gnome.desktop.screensaver picture-uri".to_string(), file_wallpaper_path);
        gnome::write_settings(&"org.gnome.desktop.screensaver picture-options".to_string(), &picture_option);
    } else if display_manager.contains("cinnamon") {
        gnome::write_settings(&"org.cinnamon.desktop.background picture-uri".to_string(), file_wallpaper_path);
        gnome::write_settings(&"org.cinnamon.desktop.background picture-options".to_string(), &picture_option);
    } else if display_manager.contains("deepin") {
        gnome::write_settings(&"com.deepin.wrap.gnome.desktop.background picture-uri".to_string(), file_wallpaper_path);
        gnome::write_settings(&"com.deepin.wrap.gnome.desktop.background picture-options".to_string(), &picture_option);
        gnome::write_settings(&"com.deepin.wrap.gnome.desktop.screensaver picture-uri".to_string(), file_wallpaper_path);
        gnome::write_settings(&"com.deepin.wrap.gnome.desktop.screensaver picture-options".to_string(), &picture_option);
    } else if display_manager.contains("plasma") || display_manager.contains("kde") {
        executeCommand(createKdePlasmaChangeWallpaperCommand(wallpaperFileName));
        executeCommand(createKdePlasmaChangeLockscreenCommand(wallpaperFileName));
    } else if display_manager.contains("xfce") {
        if (QFile::exists("/bin/feh") || QFile::exists("usr/bin/feh")) {
            executeCommand("feh --bg-scale " + wallpaperFileName);
        } else {
            setXfceWallpaper(wallpaperFileName);
        }
    } else {
        qInfo() << display_manager << " is not supported yet.";
    }
}

fn write_gnome_settings(key: &String, value: &String) {
    if (!isGSettingsValueEquals(key, value)) {
        executeCommand("gsettings set " + key + " " + value);
    }
}

fn get_display_manager() -> String {
    return env::var("XDG_CURRENT_DESKTOP").unwrap().trim().to_lowercase();
}
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use dirs;
use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::cli;

pub fn change_wallpaper(image_data: &Vec<u8>) {
    clear_wallpaper_dir();
    let file_path = persists_to_file(image_data);
    set_wallpaper(file_path);
}

fn clear_wallpaper_dir() {
    let path: PathBuf = [dirs::home_dir().unwrap().to_str().unwrap(), ".wallpaper"].iter().collect();
    std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path);
}

fn persists_to_file(image_data: &Vec<u8>) -> String {
    let path = build_target_path();
    let mut target_file = File::create(path.as_str()).expect("Unable to create file");
    target_file.write_all(image_data).expect("Unable to write data");
    return path;
}

fn build_target_path() -> String {
    let user_home = dirs::home_dir().unwrap();
    let file_name = [
        rand::thread_rng().sample_iter(&Alphanumeric).take(10).collect::<String>(),
        ".jpg".to_string()].join("");
    let path: PathBuf = [user_home.to_str().unwrap(), ".wallpaper", file_name.as_str()].iter().collect();
    return path.to_str().unwrap().to_string();
}

fn set_wallpaper(file_path: String) {
    let change_kde_wallpaper_cmd = [
        "dbus-send --session --dest=org.kde.plasmashell --type=method_call /PlasmaShell org.kde.PlasmaShell.evaluateScript 'string:",
        "var Desktops = desktops();",
        "for (i=0;i<Desktops.length;i++) {",
        "        d = Desktops[i];",
        "        d.wallpaperPlugin = \"org.kde.image\";",
        "        d.currentConfigGroup = Array(\"Wallpaper\",",
        "                                    \"org.kde.image\",",
        "                                    \"General\");",
        "        d.writeConfig(\"Image\", \"file://", file_path.as_str(), "\");",
        "        d.writeConfig(\"FillMode\", 1);",
        "}'"].join("");
    cli::execute_command(change_kde_wallpaper_cmd.to_string());
}
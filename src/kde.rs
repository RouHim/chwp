use crate::cli;

/// Sets the wallpaper to the given image path
/// # Arguments
/// * `file_path` - The path to the image to set as the wallpaper
pub fn set_wallpaper(file_path: &str) {
    let change_kde_wallpaper_cmd = [
        "dbus-send --session --dest=org.kde.plasmashell --type=method_call /PlasmaShell org.kde.PlasmaShell.evaluateScript 'string:",
        "var Desktops = desktops();",
        "for (i=0;i<Desktops.length;i++) {",
        "        d = Desktops[i];",
        "        d.wallpaperPlugin = \"org.kde.image\";",
        "        d.currentConfigGroup = Array(\"Wallpaper\",",
        "                                    \"org.kde.image\",",
        "                                    \"General\");",
        "        d.writeConfig(\"Image\", \"file://", file_path, "\");",
        "        d.writeConfig(\"FillMode\", 1);",
        "}'"].join("");
    cli::execute_command(&change_kde_wallpaper_cmd);
}

/// Sets the lock screen wallpaper to the given image path
/// # Arguments
/// * `file_path` - The path to the image to set as the wallpaper
pub fn set_lockscreen(file_path: &str) {
    let change_kde_lockscreen_cmd = [
        "kwriteconfig5 --file ~/.config/kscreenlockerrc ",
        "--group Greeter ",
        "--group Wallpaper ",
        "--group org.kde.image ",
        "--group General ",
        "--key Image \"",
        file_path,
        "\"",
    ]
    .join("");
    cli::execute_command(&change_kde_lockscreen_cmd);
}

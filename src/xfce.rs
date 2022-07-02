use crate::cli;

/// Sets the wallpaper to the given image
/// # Arguments
/// * `wallpaper_path` - The path to the image to set as the wallpaper
pub fn set_wallpaper(wallpaper_path: &str) {
    let channels = cli::execute_command("xfconf-query -c xfce4-desktop -l | grep \"last-image$\"");
    let channel_list = channels.split('\n');

    for channel in channel_list {
        cli::execute_command(
            format!(
                "xfconf-query --channel xfce4-desktop --property {channel} --set {wallpaper_path}"
            )
            .as_str(),
        );
    }
}

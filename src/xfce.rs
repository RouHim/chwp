use crate::cli;

/// Sets the wallpaper to the given image
/// # Arguments
/// * `wallpaper_path` - The path to the image to set as the wallpaper
pub fn set_wallpaper(wallpaper_path: &String) {
    let channels = cli::execute_command(
        &"xfconf-query -c xfce4-desktop -l | grep \"last-image$\"".to_string(),
    );
    let channel_list = channels.split('\n');

    for channel in channel_list {
        cli::execute_command(
            &[
                "xfconf-query --channel xfce4-desktop --property ",
                channel,
                " --set ",
                wallpaper_path,
            ]
            .join(""),
        );
    }
}

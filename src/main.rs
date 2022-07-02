extern crate image;
extern crate rand;

use std::env;

mod cli;
mod config;
mod display;
mod display_manager;
mod download;
mod filesystem;
mod gnome;
mod image_processor;
mod kde;
mod pixabay;
mod xfce;

fn main() {
    // get args with app path
    let args: Vec<String> = env::args().skip(1).collect();

    // parse the args to get a configuration
    let config = config::parse_cli_args(args);

    // read the display info
    let display_info = display::get_info();

    // retrieve the image data from pixabay
    let image_data = if config::is_url(&config.query) {
        download::get_data(&config.query)
    } else if config::is_local_path(&config.query) {
        filesystem::read_file(&config.query)
    } else {
        pixabay::get_image_data(&config, &display_info)
    };

    // scale the image to fit the display
    let image = image_processor::scale_image(image_data, config.span, &display_info);

    // change the wallpaper to the scaled image
    display_manager::change_wallpaper(image, &config);
}

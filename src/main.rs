use std::env;

use crate::config::parse_cli;
use crate::display::get_display_info;

mod config;
mod display;

fn main() {
    // get args with app path
    let args: Vec<String> = env::args().skip(1).collect();

    // parse the args to get a configuration
    let config = parse_cli(args);

    // read the display info
    let displayInfo = get_display_info();

    // retrieve the image url from pixabay

    // download the url

    // scale the image to fit the display

    // change the wallpaper to the scaled image
}

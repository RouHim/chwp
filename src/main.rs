use std::env;

mod config;
mod display;
mod pixabay;

fn main() {
    // get args with app path
    let args: Vec<String> = env::args().skip(1).collect();

    // parse the args to get a configuration
    let config = config::parse_cli(args);

    // read the display info
    let display_info = display::get_display_info();

    // retrieve the image url from pixabay
    let image_url = pixabay::get_image_url(config, display_info);
    println!("{}", image_url);

    // download the url

    // scale the image to fit the display

    // change the wallpaper to the scaled image
}

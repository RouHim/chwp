use std::env;
use crate::kde::execute_kde_command;

mod config;
mod display;
mod pixabay;
mod image;
mod kde;

fn main() {
    // // get args with app path
    // let args: Vec<String> = env::args().skip(1).collect();
    //
    // // parse the args to get a configuration
    // let config = config::parse_cli(args);
    //
    // // read the display info
    // let display_info = display::get_display_info();
    //
    // // retrieve the image url from pixabay
    // let mut image_data = pixabay::get_image_data(&config, &display_info);
    //
    // // scale the image to fit the display
    // image_data = image::scale_image(&image_data, config.span.clone(), &display_info);
    // println!("{} bytes", image_data.len());
    //
    // // change the wallpaper to the scaled image
    // kde::change_wallpaper(&image_data);




    println!("{}", execute_kde_command(r#"print("test");"#.to_string()));
}

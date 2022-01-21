use std::{env, process};
extern crate timer;

use gtk::{glib::timeout_add_seconds, prelude::Continue};
use widentify::{load_css, create_window, Arguments, load_config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let config = load_config().expect("Configuration error");

    if gtk::init().is_err() { //Initialize Gtk before doing anything with it
        panic!("Can't init GTK");
    }

    load_css();


    // Create Left Window
    create_window("Left Workspace", &arguments.left_announcement, config.left_width, config.left_height, config.left_x_pos, config.left_y_pos);

    // Create Center Window
    create_window("Center Workspace", &arguments.center_announcement, config.center_width, config.center_height, config.center_x_pos, config.center_y_pos);

    // Create Right Window
    create_window("Right Workspace",&arguments.right_announcement, config.right_width, config.right_height, config.right_x_pos, config.right_y_pos);
    
    timeout_add_seconds(3, || {gtk::main_quit(); Continue(false)});

    gtk::main();
}


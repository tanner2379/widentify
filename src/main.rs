use std::env;
extern crate timer;

use gtk::{glib::timeout_add_seconds, prelude::Continue};
use widentify::{load_css, create_window, load_config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = load_config().expect("Configuration error");

    if gtk::init().is_err() { //Initialize Gtk before doing anything with it
        panic!("Can't init GTK");
    }

    load_css();


    match args.len() {
        2 => {
            // Create Left Window
            create_window(&config.left_title, &args[1], config.left_width, config.left_height, config.left_x_pos, config.left_y_pos);
        },
        3 => {
            // Create Left Window
            create_window(&config.left_title, &args[1], config.left_width, config.left_height, config.left_x_pos, config.left_y_pos);

            // Create Center Window
            create_window(&config.center_title, &args[2], config.center_width, config.center_height, config.center_x_pos, config.center_y_pos);
        },
        4 => {
            // Create Left Window
            create_window(&config.left_title, &args[1], config.left_width, config.left_height, config.left_x_pos, config.left_y_pos);

            // Create Center Window
            create_window(&config.center_title, &args[2], config.center_width, config.center_height, config.center_x_pos, config.center_y_pos);

            // Create Right Window
            create_window(&config.right_title, &args[3], config.right_width, config.right_height, config.right_x_pos, config.right_y_pos);
        },
        _ => {
            if args.len() < 2 {
                panic!("Needs at least one argument.\nExample: widentify 1 2 3");
            } else {
                panic!("This program only handles up to 3 arguments");
            }
        }
    }
    
    if config.timeout != 0 {
        timeout_add_seconds(config.timeout, || {gtk::main_quit(); Continue(false)});
    }

    gtk::main();
}


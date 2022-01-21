use gtk::gdk::Screen;
use gtk::{prelude::*, CssProvider, StyleContext, Window, WindowType, Label};
use std::{env, fs, error::Error};

pub struct Config {
    pub timeout: u32,

    pub left_title: String,
    pub left_x_pos: i32,
    pub left_y_pos: i32,
    pub left_width: i32,
    pub left_height: i32,

    pub center_title: String,
    pub center_x_pos: i32,
    pub center_y_pos: i32,
    pub center_width: i32,
    pub center_height: i32,

    pub right_title: String,
    pub right_x_pos: i32,
    pub right_y_pos: i32,
    pub right_width: i32,
    pub right_height: i32,
}

pub fn create_window(title: &str, message: &str, width: i32, height: i32, x_pos: i32, y_pos: i32) {
    let window = Window::new(WindowType::Toplevel); 

    //Destroy window on exit
    window.connect_delete_event(|_,_| {gtk::main_quit(); Inhibit(false) });

    window.set_title(title);
    window.set_default_size(width, height);
    let label = Label::new(Some(message));
    window.add(&label);
    window.show_all();
    window.move_(x_pos, y_pos);
}

pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let mut timeout: u32 = 3;

    let mut left_title: String = String::from("Widentify");
    let mut left_x_pos: i32 = 1760;
    let mut left_y_pos: i32 = 10;
    let mut left_width: i32 = 150;
    let mut left_height: i32 = 150;

    let mut center_title: String = String::from("Widentify");
    let mut center_x_pos: i32 = 3680;
    let mut center_y_pos: i32 = 10;
    let mut center_width: i32 = 150;
    let mut center_height: i32 = 150;

    let mut right_title: String = String::from("Widentify");
    let mut right_x_pos: i32 = 4760;
    let mut right_y_pos: i32 = 10;
    let mut right_width: i32 = 150;
    let mut right_height: i32 = 150;

    if let Ok(mut path) = env::var("HOME").to_owned() {
        path.push_str("/.config/widentify/widentify.config");

        if let Ok(contents) = fs::read_to_string(path) {

            let lines: Vec<&str> = contents.lines().collect();

            for line in lines {
                let split: Vec<&str> = line.split(": ").collect();

                match split[0] {
                    "timeout" => timeout = split[1].parse::<u32>().unwrap(),

                    "left_title" => left_title = String::from(split[1]),
                    "left_x_pos" => left_x_pos = split[1].parse::<i32>().unwrap(),
                    "left_y_pos" => left_y_pos = split[1].parse::<i32>().unwrap(),
                    "left_width" => left_width = split[1].parse::<i32>().unwrap(),
                    "left_height" => left_height = split[1].parse::<i32>().unwrap(),

                    "center_title" => center_title = String::from(split[1]),
                    "center_x_pos" => center_x_pos = split[1].parse::<i32>().unwrap(),
                    "center_y_pos" => center_y_pos = split[1].parse::<i32>().unwrap(),
                    "center_width" => center_width = split[1].parse::<i32>().unwrap(),
                    "center_height" => center_height = split[1].parse::<i32>().unwrap(),

                    "right_title" => right_title = String::from(split[1]),
                    "right_x_pos" => right_x_pos = split[1].parse::<i32>().unwrap(),
                    "right_y_pos" => right_y_pos = split[1].parse::<i32>().unwrap(),
                    "right_width" => right_width = split[1].parse::<i32>().unwrap(),
                    "right_height" => right_height = split[1].parse::<i32>().unwrap(),
                    _ => ()
                }
            }
        } else {
            println!("Config file not found, using default values");
        };
        
        Ok( Config {
            timeout,
            left_title,
            left_x_pos,
            left_y_pos,
            left_width,
            left_height,
            center_title,
            center_x_pos,
            center_y_pos,
            center_width,
            center_height,
            right_title,
            right_x_pos,
            right_y_pos,
            right_width,
            right_height
        })
    } else {
        println!("HOME environment variable not set, using default config");

        Ok( Config {
            timeout,
            left_title,
            left_x_pos,
            left_y_pos,
            left_width,
            left_height,
            center_title,
            center_x_pos,
            center_y_pos,
            center_width,
            center_height,
            right_title,
            right_x_pos,
            right_y_pos,
            right_width,
            right_height
        })
    }
}

pub fn load_css() {
    if let Ok(mut path) = env::var("HOME").to_owned() {
        let config_path = "/.config/widentify/widentify.css";

        path.push_str(config_path);

        let provider = CssProvider::new();

        match provider.load_from_path(&path) {
            Ok(_) => {
                // Add the provider to the default screen
                StyleContext::add_provider_for_screen(
                    &Screen::default().expect("Could not connect to a display."),
                    &provider,
                    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
            },
            Err(_) => println!("CSS File not found")
        }
    } else {
        println!("HOME environment variable not set, using empty styling");
    }
}

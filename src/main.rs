extern crate toml;

use toml::Value;
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};

fn main() {
    let mut file = File::open("config/config.toml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let value = contents.parse::<Value>().unwrap();
    let five_seconds = time::Duration::new(5, 0);

    loop {
        if value["cfg"]["leader"].as_bool().unwrap() {
            println!("I am a leader");
        } else {
            println!("I am a follower");
        }
        if let Some(backend) = value["cfg"]["backend"].as_str() {
            println!("my backend leader is {}", backend);
        }
        thread::sleep(five_seconds);
    }
}

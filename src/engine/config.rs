use json::JsonValue;
use std::fs::File;
use std::io::Read;

pub(crate) struct Config;

impl Config {
    pub fn get(name:&str) -> JsonValue {
        let mut f = File::open("config/"+*name)?;
        println!("{}", "config/"+*name);
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        json::parse(path).unwrap()
    }
}
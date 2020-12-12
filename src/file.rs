// Implements struct for interacting with configuration files.

use std::fs;

pub struct ConfigFile<'a> {
    pub name: &'a str,
    pub data: json::JsonValue,
}

impl ConfigFile<'_> {
    pub fn parse<'a>(path: &'a str) -> ConfigFile {
        let contents = fs::read_to_string(&path).expect("Something went wrong reading the file");
        ConfigFile {
            name: &path,
            data: json::parse(&contents).unwrap(),
        }
    }
}

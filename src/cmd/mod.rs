use super::file::ConfigFile;

pub fn dialog() {}

pub fn input() {}

pub fn tell() {}

pub fn say() {}

pub fn get<'a>(variable: &str, variables: &'a json::JsonValue) -> &'a str {
    if variable == "%in%" {
        "%in% keyword in developing"
    } else {
        println!("{}", variable);
        variables[variable].as_str().unwrap_or("")
    }
}

pub fn set() {}

pub fn act() {}

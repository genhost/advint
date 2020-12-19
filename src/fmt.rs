// Implements functions for formating config file values

use super::{cmd, file::ConfigFile};

use unicode_segmentation::UnicodeSegmentation;

pub fn simple_fmt(src_str: &str, fmt_str: &str) -> String {
    let src_str_vec = fmt_str.split_word_bounds().collect::<Vec<&str>>();
    let mut end_str_vec: Vec<&str> = Vec::new();
    let mut next = false;

    for word in src_str_vec {
        if word == "{" {
            next = true;
        } else if next && word == "}" {
            end_str_vec.push(&src_str);
        } else {
            end_str_vec.push(&word);
        }
    }

    end_str_vec.join("")
}

pub fn match_get_cmd<'a>(fmt_key: &str, vars: &json::JsonValue) -> String {
    let mut end: Vec<&str> = Vec::new();
    let mut cmd: Vec<&str> = Vec::new();
    let mut cmd_ptr = false;

    for char in fmt_key.split_word_bounds().collect::<Vec<&str>>() {
        if char == "}" {
            cmd_ptr = false;
            if cmd.join("").split(":").collect::<Vec<&str>>()[0] == "get" {
                end.push(cmd::get(
                    cmd.join("").split(":").collect::<Vec<&str>>()[1],
                    vars,
                ))
            }
        } else if cmd_ptr {
            cmd.push(char);
        } else if char == "{" {
            cmd_ptr = true;
        } else {
            end.push(char);
        }
    }

    end.join("")
}

pub fn match_cmd<'a>(fmt_key: &str, fmt_val: Option<&str>, adv: &ConfigFile) -> String {
    let fmt_val = fmt_val.unwrap_or("");
    let mut end: Vec<&str> = Vec::new();
    let mut cmd: Vec<&str> = Vec::new();
    let mut cmd_ptr_found = false;

    for char in fmt_key.split_word_bounds().collect::<Vec<&str>>() {
        if char == "}" {
            cmd_ptr_found = false;
            match cmd.join("").split(":").collect::<Vec<&str>>()[0] {
                "get" => end.push(cmd::get(
                    cmd.join("").split(":").collect::<Vec<&str>>()[1],
                    &adv.data["variables"],
                )),
                "say" => println!("{}", fmt_val),
                _ => (),
            }
        } else if cmd_ptr_found {
            cmd.push(char);
        } else if char == "{" {
            cmd_ptr_found = true;
        } else {
            end.push(char);
        }
    }

    end.join("")
}

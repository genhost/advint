// Implements functions for deep interaction with strings.

use unicode_segmentation::UnicodeSegmentation;

pub fn len(string: &str) -> usize {
    UnicodeSegmentation::graphemes(string, true)
        .collect::<Vec<&str>>()
        .len()
}

pub fn cut(src_str: &str, length: usize) -> String {
    if length < len(src_str) {
        UnicodeSegmentation::graphemes(src_str, true).collect::<Vec<&str>>()[..length].join("")
    } else {
        String::from(src_str)
    }
}

pub fn cut_json(string: &json::JsonValue, length: &json::JsonValue) -> String {
    cut(
        string.as_str().unwrap_or_default(),
        length.as_usize().unwrap_or(string.len()),
    )
}

pub fn fmt(src_str: &str, fmt_str: &str) -> String {
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

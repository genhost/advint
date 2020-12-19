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

#![allow(dead_code)]
use regex::Regex;
use crossterm::terminal::size as terminal_size;
pub struct Format {}
impl Format {
    pub fn center(width: u32, text: &str) -> String {
        let text_len = Format::strlen_no_color(text);
        let padding = (width - text_len) / 2;
        let mut centered_text = String::new();
        Format::pad(&mut centered_text, padding);
        centered_text.push_str(text);
        Format::pad(&mut centered_text, padding);
        centered_text
    }
    pub fn stretch(width: u32, str1: &str, str2: &str) -> String {
        let len1 = Format::strlen_no_color(str1); let len2 = Format::strlen_no_color(str2);
        let mut stretched_text = String::new();
        stretched_text.push_str(str1);
        Format::pad(&mut stretched_text, width - len1 - len2);
        stretched_text.push_str(str2);
        stretched_text
    }
    pub fn right(width: u32, text: &str) -> String {
        let text_len = Format::strlen_no_color(text);
        let padding = width - text_len;
        let mut right_text = String::new();
        Format::pad(&mut right_text, padding);
        right_text.push_str(text);
        right_text
    }
    pub fn left(width: u32, text: &str) -> String {
        let text_len = Format::strlen_no_color(text);
        let padding = width - text_len;
        let mut left_text = String::new();
        left_text.push_str(text);
        Format::pad(&mut left_text, padding);
        left_text
    }
    pub fn console_width() -> u32 {
        terminal_size().unwrap().0 as u32
    }
    fn strlen_no_color(text: &str) -> u32 {
        let re = Regex::new(r"\x1b\[[\d;]*m").unwrap();
        re.replace_all(text, "").len() as u32
    }
    fn pad(str: &mut String, padding: u32) {
        for _ in 0..padding {
            str.push(' ');
        }
    }
}
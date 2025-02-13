#![allow(dead_code)]
pub struct Chalk {
}
impl Chalk {
    pub fn colorize(text: &str, color: u32, bold: bool) -> String {
        format!("\x1b[{};38;2;{};{};{}m{}\x1b[0m", if bold { 1 } else { 0 }, (color >> 16) & 0xff, (color >> 8) & 0xff, color & 0xff, text)
    }
}
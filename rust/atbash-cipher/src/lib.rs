#![feature(ascii_ctype)]
use std::ascii::AsciiExt;

pub fn encode(input: &str) -> String {
    decode(input)
        .chars()
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|w| w.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn decode(input: &str) -> String {
    input
        .chars()
        .into_iter()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .map(swap_char)
        .collect()
}

const UPPER: u8 = 'z' as u8;
const LOWER: u8 = 'a' as u8;

fn swap_char(c: char) -> char {
    if c.is_digit(10) {
        c
    } else {
        (UPPER + LOWER - c as u8) as char
    }
}

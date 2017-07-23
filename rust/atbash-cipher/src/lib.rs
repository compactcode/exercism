#![feature(ascii_ctype)]
use std::ascii::AsciiExt;

pub fn encode(input: &str) -> String {
    substitue(input)
}

pub fn decode(input: &str) -> String {
    substitue(input).replace(" ", "")
}

fn substitue(input: &str) -> String {
    input
        .chars()
        .into_iter()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .map(translate)
        .fold(String::new(), |mut encoded, c| {
            if encoded.len() % 6 == 0 {
                encoded.push(' ');
            }
            encoded.push(c);
            encoded
        })
        .trim()
        .to_string()
}

const UPPER: u8 = 'z' as u8;
const LOWER: u8 = 'a' as u8;

fn translate(c: char) -> char {
    if c.is_digit(10) {
        c
    } else {
        (UPPER + LOWER - c as u8) as char
    }
}

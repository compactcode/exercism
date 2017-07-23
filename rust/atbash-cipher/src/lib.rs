#![feature(ascii_ctype)]
use std::ascii::AsciiExt;

pub fn encode(input: &str) -> String {
    substitue(input)
}

pub fn decode(input: &str) -> String {
    substitue(input).replace(" ", "")
}

fn substitue(input: &str) -> String {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    input
        .chars()
        .into_iter()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .map(|c| {
            match alphabet.iter().position(|&a| a == c) {
                Some(index) => alphabet[25 - index],
                _ => c
            }
        })
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

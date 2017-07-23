#![feature(ascii_ctype)]
use std::ascii::AsciiExt;

pub fn encode(input: &str) -> String {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut spacer_count = 0;
    input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| {
            match alphabet.iter().position(|&a| a == c) {
                Some(index) => alphabet[25 - index],
                _ => c
            }
        })
        .fold(String::new(), |mut encoded, c| {
            encoded.push(c);
            spacer_count += 1;
            if spacer_count == 5 {
                encoded.push(' ');
                spacer_count = 0;
            }
            encoded
        })
        .trim()
        .to_string()
}

pub fn decode(input: &str) -> String {
    encode(input).replace(" ", "")
}

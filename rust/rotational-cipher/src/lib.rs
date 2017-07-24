static ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

fn rotate_char(c: char, key: usize) -> char {
    let lower: Vec<char> = ALPHABET.chars().collect();
    let upper: Vec<char> = lower.iter().flat_map(|c| c.to_uppercase()).collect();
    if c.is_lowercase() {
        match lower.iter().position(|&a| a == c) {
            Some(index) => lower[(key + index) % 26],
            _ => c
        }
    } else {
        match upper.iter().position(|&a| a == c) {
            Some(index) => upper[(key + index) % 26],
            _ => c
        }
    }
}

pub fn rotate(input: &str, key: usize) -> String {
    input
        .chars()
        .map(|c| rotate_char(c, key))
        .collect()
}

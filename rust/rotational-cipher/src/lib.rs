pub fn rotate(input: &str, key: u8) -> String {
    input
        .chars()
        .map(|c| rotate_char(c, key))
        .collect()
}

fn rotate_char(c: char, key: u8) -> char {
    match c {
        'a'...'z' => rotate_from_base(c as u8, 'a' as u8, key),
        'A'...'Z' => rotate_from_base(c as u8, 'A' as u8, key),
        _ => c
    }
}

fn rotate_from_base(c: u8, base: u8, key: u8) -> char {
    ((c - base + key) % 26 + base) as char
}

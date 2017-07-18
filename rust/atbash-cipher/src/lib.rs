pub fn encode(input: &str) -> String {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| {
            alphabet[25 - alphabet.iter().position(|&a| a == c).unwrap()]
        })
        .collect::<String>()
}

pub fn decode(input: &str) -> String {
    input.to_string()
}

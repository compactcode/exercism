pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split_terminator(|x: char| x.is_whitespace() || x == '-')
        .fold(Vec::new(), |mut result, word| {
            let (first, rest) = word.split_at(1);
            result.push(first.to_string());
            if !rest.chars().filter(|x| x.is_alphabetic()).all(char::is_uppercase) {
                result.push(rest.chars().filter(|x| x.is_uppercase()).collect::<String>());
            }
            result
        })
        .join("")
        .to_uppercase()
}

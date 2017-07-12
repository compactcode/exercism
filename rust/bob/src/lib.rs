pub fn reply(input: &str) -> &str {
    fn is_only_uppercase(x: &str) -> bool {
        x.chars().any(char::is_alphabetic) && !x.chars().any(char::is_lowercase)
    }
    match input.trim() {
        x if x.is_empty() => "Fine. Be that way!",
        x if is_only_uppercase(x) => "Whoa, chill out!",
        x if x.ends_with("?") => "Sure.",
        _ => "Whatever.",
    }
}

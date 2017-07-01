pub fn reply(input: &str) -> &str {
    match input {
        x if x.is_empty() => "Fine. Be that way!",
        x if x.ends_with("?") => "Sure.",
        x if x.find(char::is_lowercase) == None => "Whoa, chill out!",
        _ => "Whatever.",
    }
}

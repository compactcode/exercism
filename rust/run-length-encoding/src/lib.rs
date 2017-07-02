pub fn encode(content: &str) -> String {
    let mut result = String::new();
    let mut chars = content.chars().peekable();
    let mut curr_count = 0;
    while let Some(curr) = chars.next() {
        curr_count += 1;
        if chars.peek() != Some(&curr) {
            match curr_count {
                1 => result.push(curr),
                _ => result.push_str(&format!("{}{}", curr_count, curr))
            }
            curr_count = 0;
        }
    }
    result
}

pub fn decode(content: &str) -> String {
    let mut result = String::new();
    let mut numeric_chars = String::new();
    for curr in content.chars()  {
        if curr.is_numeric() {
            numeric_chars.push(curr)
        } else {
            match numeric_chars.parse::<usize>() {
                Ok(total) => result.push_str(&curr.to_string().repeat(total)),
                Err(_)    => result.push(curr)
            }
            numeric_chars.clear()
        }
    }
    result
}

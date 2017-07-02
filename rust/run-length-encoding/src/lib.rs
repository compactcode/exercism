pub fn encode(content: &str) -> String {
    let mut result = String::new();
    let mut chars = content.chars().peekable();
    let mut curr_count = 0;
    while let Some(curr) = chars.next() {
        curr_count += 1;
        if chars.peek() != Some(&curr) {
            if curr_count > 1 {
                result.push_str(&curr_count.to_string())
            }
            result.push(curr);
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
            result.push_str(&curr.to_string().repeat(
                numeric_chars.parse::<usize>().unwrap_or(1)
            ));
            numeric_chars.clear()
        }
    }
    result
}

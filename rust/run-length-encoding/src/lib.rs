pub fn encode(content: &str) -> String {
    let mut result = String::new();
    let mut chars = content.chars().peekable();
    let mut curr_count = 0;
    loop {
        match chars.next() {
            Some(curr) => {
                curr_count += 1;
                match chars.peek() {
                    Some(n) if n == &curr => {
                        // Do nothing.
                    }
                    _ => {
                        match curr_count {
                            1 => result.push(curr),
                            _ => result.push_str(&format!("{}{}", curr_count, curr))
                        }
                        curr_count = 0;
                    }
                }
            },
            None => break
        }
    }
    result
}

pub fn decode(content: &str) -> String {
    String::from(content)
}

fn double(n: u32) -> u32 {
    if n * 2 > 9 {
        n * 2 - 9
    } else {
        n * 2
    }
}

pub fn is_valid(input: &str) -> bool {
    let chars: Vec<char> = input.chars().filter(|&c| !char::is_whitespace(c)).collect();

    if chars.len() < 2 {
        return false
    }

    if chars.iter().cloned().filter(|&c| !char::is_numeric(c)).count() > 0 {
        return false
    }

    chars
        .iter()
        .rev()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .map(|(i, n)|
            if i % 2 == 1 {
                double(n)
            } else {
                n
            }
        )
        .sum::<u32>() % 10 == 0
}

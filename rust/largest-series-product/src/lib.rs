pub fn lsp(input: &str, span: usize) -> Result<u32, ()> {
    if span == 0 {
        return Ok(1)
    }

    if input.len() < span || input.chars().any(char::is_alphabetic) {
        return Err(())
    }

    let mut best    = 0;
    let mut window = Vec::new();

    for n in input.chars().flat_map(|c| c.to_digit(10)) {
        window.push(n);
        if window.len() == span {
            if window.iter().product::<u32>() > best {
                best = window.iter().product();
            }
            window.remove(0);
        }
    }

    Ok(best)
}

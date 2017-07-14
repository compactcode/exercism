pub fn lsp(input: &str, span: usize) -> Result<u32, ()> {
    if span == 0 {
        return Ok(1)
    }

    if input.len() < span || input.chars().any(char::is_alphabetic) {
        return Err(())
    }

    input
        .chars()
        .flat_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>()
        .windows(span)
        .map(|window| window.iter().product())
        .max()
        .ok_or(())
}

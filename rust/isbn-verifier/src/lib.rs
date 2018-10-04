/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let numbers = isbn
        .chars()
        .filter_map(|x| to_number(x))
        .collect::<Vec<u32>>();

    if numbers.len() != 10 {
        return false;
    }

    match numbers.iter().position(|x| *x == 10) {
        Some(9) => (),
        Some(_) => return false,
        None    => (),
    }

    return numbers
        .iter()
        .enumerate()
        .map(|(index, number)| (10 - index as u32) * number)
        .sum::<u32>()
        .wrapping_rem(11) == 0
}

fn to_number(x: char) -> Option<u32> {
    match x {
        'X' => Some(10),
        _   => x.to_digit(10)
    }
}

pub fn number(input: &str) -> Option<String> {
    let mut numbers: Vec<char> = input.chars().filter(|x| x.is_numeric()).collect();

    match numbers.len() {
      10 => validate_numbers(numbers),
      11 => {
          match numbers.remove(0) {
              '1' => validate_numbers(numbers),
              _   => None
          }
      },
      _  => None
    }
}

fn validate_numbers(numbers: Vec<char>) -> Option<String> {
    if numbers[0] > '1' && numbers[3] > '1' {
        Some(numbers.iter().collect())
    } else {
        None
    }
}

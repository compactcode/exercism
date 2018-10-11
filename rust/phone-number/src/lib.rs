pub fn number(input: &str) -> Option<String> {
    let mut numbers: String = input.chars().filter(|x| x.is_numeric()).collect();

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

fn validate_numbers(numbers: String) -> Option<String> {
    match numbers.get(0..1) {
        Some("0") | Some("1") => return None,
        _                     => ()
    }

    match numbers.get(3..4) {
        Some("0") | Some("1") => None,
        _                     => Some(numbers)
    }
}

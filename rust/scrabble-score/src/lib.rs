use std::ascii::AsciiExt;

pub fn score(word: &str) -> usize {
    word.to_uppercase().chars().filter(char::is_ascii).map(char_score).sum()
}

fn char_score(c: char) -> usize {
    match c {
        'D' | 'G'                   => 2,
        'B' | 'C' | 'M' | 'P'       => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K'                         => 5,
        'J' | 'X'                   => 8,
        'Q' | 'Z'                   => 10,
        _                           => 1
    }
}

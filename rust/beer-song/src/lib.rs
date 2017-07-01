fn bottles(bottles: u8) -> String {
    match bottles {
        1 => String::from("1 bottle"),
        _ => format!("{} bottles", bottles),
    }
}

pub fn verse(number: u8) -> String {
    match number {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        _ => format!("{0} of beer on the wall, {0} of beer.\nTake one down and pass it around, {1} of beer on the wall.\n", bottles(number), bottles(number - 1)),
    }
}

pub fn sing(finish: u8, start: u8) -> String {
    (start..finish + 1).rev().map(verse).collect::<Vec<String>>().join("\n")
}

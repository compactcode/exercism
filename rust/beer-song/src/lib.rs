pub fn verse(number: u32) -> String {
    match number {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"),
        _ => String::from(format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", number, number - 1)),
    }
}

pub fn sing(finish: u32, start: u32) -> String {
    let mut result : String = String::new();

    for i in (start..finish + 1).rev() {
        result.push_str(&verse(i));
        result.push('\n');
    }

    result.pop();
    result
}

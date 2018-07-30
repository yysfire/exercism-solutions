pub fn verse(n: i32) -> String {
    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", 2, 2, 1),
        m => {
            if m < 0 {
                String::new()
            } else {
                format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", m, m, m - 1)
            }
        }
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut result = String::new();
    if start < end {
        return result;
    }

    for n in (end..=start).into_iter().rev() {
        result.push_str(&verse(n));
        if n >= 0 && n != end {
            result.push('\n');
        }
    }

    result
}

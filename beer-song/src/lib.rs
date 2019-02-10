pub fn verse(n: i32) -> String {
    let mut out = String::from("");
    match n {
        0 => out = String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => out = String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => out = String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"),
        2...99 => out = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1),
        _ => out = String::from(""),
    }
    out
}

pub fn sing(start: i32, end: i32) -> String {
    let mut out = String::from("");
    for i in (end+1..start+1).rev() {
        out = format!("{}{}{}", out, verse(i), "\n");
    }
    out = format!("{}{}", out, verse(end));
    out
}

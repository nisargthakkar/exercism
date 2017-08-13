pub fn verse(verse: u32) -> String {
    match verse {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_owned(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_owned(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_owned(),
        n => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, n-1).to_owned(),
    }
}

pub fn sing(start_verse: u32, end_verse: u32) -> String {
    let mut song = String::new();
    song.push_str(&verse(start_verse));
    for i in (end_verse..start_verse).rev() {
        song.push_str("\n");
        song.push_str(&verse(i));
    }

    song
}
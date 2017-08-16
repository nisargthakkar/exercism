pub fn rotate(text: &str, n: u8) -> String {
    let mut encoded_string: String = String::new();
    for c in text.chars() {
        let char_to_push = match c {
            'a'...'z' => ((c as u8 - 97 + n) % 26 + 97) as char,
            'A'...'Z' => ((c as u8 - 65 + n) % 26 + 65) as char,
            n => n,
        };
        encoded_string.push(char_to_push);
    }

    encoded_string
}
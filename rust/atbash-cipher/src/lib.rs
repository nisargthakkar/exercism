pub fn encode(text: &str) -> String {
    enum Step {
        EncodeAndPush,
        Push,
        Ignore
    };

    let mut encoded_string: String = String::new();
    let mut char_count = 0;
    for c in text.chars() {
        if c.is_alphanumeric() {
            let character = c.to_lowercase().next().unwrap();
            let next_step = match character {
                'a'...'z' => Step::EncodeAndPush,
                '0'...'9' => Step::Push,
                _ => Step::Ignore,
            };

            let char_pushed = match next_step {
                Step::EncodeAndPush => {
                    if char_count%5 == 0  && char_count > 0 {
                        encoded_string.push(' ');
                    }
                    encoded_string.push(encode_decode_char(character));
                    true
                },
                Step::Push => {
                    if char_count%5 == 0  && char_count > 0 {
                        encoded_string.push(' ');
                    }
                    encoded_string.push(character);
                    true
                },
                Step::Ignore => false,
            };

            if char_pushed {
                char_count += 1;
            }
        }
    }

    encoded_string
}

pub fn decode(text: &str) -> String {
    let mut encoded_string: String = String::new();
    for c in text.chars() {
        if c.is_alphanumeric() {
            let char_to_push = match c.to_lowercase().next() {
                Some(lower_char) => encode_decode_char(lower_char),
                None => c,
            };
            encoded_string.push(char_to_push);
        }
    }

    encoded_string
}

fn encode_decode_char(c: char) -> char {
    let encoding_map = vec!['z', 'y', 'x', 'w', 'v', 'u', 't', 's', 'r', 'q', 'p', 'o', 'n', 'm', 'l', 'k', 'j', 'i', 'h', 'g', 'f', 'e', 'd', 'c', 'b', 'a'];
    match c {
        'a'...'z' => encoding_map[c as usize - 97],
        n => n,
    }
}
use std::mem;

pub fn encode(text: &str) -> &'static str {
    let mut last_char: char = '\0';
    let mut char_count: u32 = 0;

    let mut encoded_string = String::new();

    for c in text.chars() {
        if c != last_char {
            encoded_string.push_str(&encode_chunk(last_char, char_count));
            last_char = c;
            char_count = 1;
        } else {
            char_count += 1;
        }
    }

    encoded_string.push_str(&encode_chunk(last_char, char_count));
    
    string_to_static_str(encoded_string.clone())
}

pub fn decode(text: &str) -> &'static str {
    enum LastCharType {
        NONE,
        DIGIT,
        CHARACTER
    };

    let mut decoded_string = String::new();

    let mut last_count: u32 = 1;
    let mut last_char: char = '\0';

    let mut last_char_type = LastCharType::NONE;

    for c in text.chars() {
        if c.is_digit(10) {
            match last_char_type {
                LastCharType::CHARACTER => {
                    decoded_string.push_str(&decode_chunk(last_char, last_count));
                    last_count = c.to_digit(10).unwrap();
                },
                LastCharType::NONE => {
                    last_count = c.to_digit(10).unwrap();
                },
                LastCharType::DIGIT => {
                    last_count = last_count * 10 + c.to_digit(10).unwrap();
                },
            }
            
            last_char_type = LastCharType::DIGIT;
        } else {
            match last_char_type {
                LastCharType::CHARACTER => {
                    decoded_string.push_str(&decode_chunk(last_char, last_count));
                    last_count = 1;
                },
                LastCharType::DIGIT => {},
                LastCharType::NONE => {
                    last_count = 1;
                },
            }

            last_char = c;
            last_char_type = LastCharType::CHARACTER;
        }
    }
    
    match last_char_type {
        LastCharType::CHARACTER | LastCharType::DIGIT => decoded_string.push_str(&decode_chunk(last_char, last_count)),
        LastCharType::NONE => {},
    }

    string_to_static_str(decoded_string.clone())
}

fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}

fn encode_chunk(last_char: char, count: u32) -> String {
    let mut encoded_chunk = String::new();
    if count > 1 {
        encoded_chunk.push_str(&count.to_string());
        encoded_chunk.push(last_char);
    } else if count == 1 {
        encoded_chunk.push(last_char);
    }

    encoded_chunk
}

fn decode_chunk(last_char: char, count: u32) -> String {
    let mut decoded_chunk = String::new();
    for _ in 0..count {
        decoded_chunk.push(last_char);
    }

    decoded_chunk
}
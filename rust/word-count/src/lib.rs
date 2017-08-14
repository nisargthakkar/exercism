use std::collections::HashMap;

pub fn word_count<'a>(s: &'a str) -> HashMap<String, u32> {
    enum CharacterType {
        LETTER,
        DIGIT,
        NONE,
    };

    enum Step {
        CONTINUE,
        STOP,
        IGNORE,
    };

    let mut m: HashMap<String, u32> = HashMap::new();

    let mut new_word = String::new();
    let mut words_vec: Vec<String> = Vec::new();

    let mut last_character = CharacterType::NONE;
    
    let mut next_step = Step::IGNORE;
    for c in s.chars() {
        let mut lowercase_character = c.to_string();
        next_step = match (last_character, c.is_digit(10), c.is_alphanumeric()) {
            (CharacterType::DIGIT, true, true) => {
                last_character = CharacterType::DIGIT;
                Step::CONTINUE
            },
            (CharacterType::LETTER, true, true) => {
                last_character = CharacterType::DIGIT;
                Step::STOP
            },
            (CharacterType::NONE, true, true) => {
                last_character = CharacterType::DIGIT;
                Step::CONTINUE
            },
            (CharacterType::DIGIT, false, true) => {
                lowercase_character = c.to_lowercase().to_string();
                last_character = CharacterType::LETTER;
                Step::STOP
            },
            (CharacterType::LETTER, false, true) => {
                lowercase_character = c.to_lowercase().to_string();
                last_character = CharacterType::LETTER;
                Step::CONTINUE
            },
            (CharacterType::NONE, false, true) => {
                lowercase_character = c.to_lowercase().to_string();
                last_character = CharacterType::LETTER;
                Step::CONTINUE
            },
            (CharacterType::DIGIT, _, false) => {
                last_character = CharacterType::NONE;
                Step::STOP
            },
            (CharacterType::LETTER, _, false) => {
                last_character = CharacterType::NONE;
                Step::STOP
            },
            (CharacterType::NONE, _, false) => {
                last_character = CharacterType::NONE;
                Step::IGNORE
            }
        };

        match next_step {
            Step::CONTINUE => new_word.push_str(&lowercase_character),
            Step::STOP => {
                words_vec.push(new_word);
                new_word = String::new();
            },
            Step::IGNORE => {},
        }
    }

    match next_step {
        Step::CONTINUE | Step::STOP => words_vec.push(new_word),
        Step::IGNORE => {},
    }

    words_vec.iter().map(|x| {
        println!("{}", x);
        let word = x.to_lowercase();

        let value = match m.get(&word) {
            Some(n) => *n as u32,
            None => 0 as u32,
        };

        m.insert(word.to_string(), value + 1)
    }).all(|_| true);

    m
}
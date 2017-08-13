pub fn reply (text: &'static str) -> &'static str {
    if is_empty(text) {
        return "Fine. Be that way!";
    }

    if is_shouting(text) {
        return "Whoa, chill out!";
    }

    if is_question(text) {
        return "Sure.";
    }

    "Whatever."
}

fn is_question(text: &'static str) -> bool {
    text.trim().ends_with("?")
}

fn is_shouting(text: &'static str) -> bool {
    let mut has_uppercase_characters = false;
    for c in text.trim().chars() {
        if c.is_lowercase() {
            return false;
        }
        if c.is_uppercase() {
            has_uppercase_characters = true;
        }
    }

    has_uppercase_characters
}

fn is_empty(text: &'static str) -> bool {
    text.trim().is_empty()
}
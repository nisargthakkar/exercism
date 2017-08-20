pub struct Brackets {
    is_valid: bool
}

fn is_opening_bracket(c: char) -> bool {
    match c {
        '{' | '[' | '(' => true,
        _ => false,        
    }
}

fn is_closing_bracket(c: char) -> bool {
    match c {
        '}' | ']' | ')' => true,
        _ => false,        
    }
}

fn brackets_match(open_bracket: &char, close_bracket: &char) -> bool {
    let open_brackets = ['{', '[', '('];
    let close_brackets = ['}', ']', ')'];

    let mut result = true;
    for i in 0..3 {
        if open_bracket == &open_brackets[i] && close_bracket != &close_brackets[i] {
            result = false;
            break;
        }
    }

    result
}

enum Step {
    Pop,
    Ignore,
}

impl Brackets {
    pub fn from(s: &str) -> Brackets {
        let (brackets, result) = s.chars().fold((Vec::new(), true), |(mut brackets, mut result), c| {
            if !result {
                return (brackets, result);
            }
            if is_opening_bracket(c) {
                brackets.push(c);
            } else if is_closing_bracket(c) {
                let next_step = match brackets.last() {
                    Some(open_bracket) => {
                        let step;
                        if brackets_match(open_bracket, &c) {
                            step = Step::Pop;
                        } else {
                            result = false;
                            step = Step::Ignore;
                        }
                        step
                    },
                    None => Step::Ignore,
                };

                match next_step {
                    Step::Pop => {brackets.pop();},
                    Step::Ignore => {},
                };
            }

            (brackets, result)
        });

        Brackets {
            is_valid: result && brackets.len() == 0
        }
    }

    pub fn are_balanced(&self) -> bool {
        self.is_valid
    }
}
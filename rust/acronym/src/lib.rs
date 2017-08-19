pub fn abbreviate(text: &'static str) -> String {
    text.split(|c: char| !c.is_alphabetic())
        .filter(|s| !s.is_empty())
        .flat_map(|w| {
            w.chars()
                .fold((Vec::new(), true), |(mut output, should_read), c| {
                    let should_read = {
                        if output.is_empty() {
                            output.push(c.to_uppercase().next().unwrap());
                            false
                        } else if is_strict_uppercase(c) && should_read {
                            output.push(c);
                            false
                        } else {
                            !is_strict_uppercase(c)
                        }
                    };

                    (output, should_read)
                }).0
        }).collect::<String>()
}

fn is_strict_uppercase(c: char) -> bool {
    c.to_lowercase().next().unwrap() != c
}
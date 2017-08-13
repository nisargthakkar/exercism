//
// See Rust Language Specific Instructions
// below normal exercise description.
//
pub fn encode(n: u64) -> String {
    let units: Vec<String> = get_units();

    if n < 1000 {
        return encode_0_999(n);
    }

    let mut batches: Vec<u64> = Vec::new();

    let mut working_n = n;
    while working_n != 0 {
        batches.push(working_n%1000);
        working_n /= 1000;
    }

    let total_units = batches.len();

    batches.reverse();

    let mut current_unit = total_units;
    let mut worded_batches: Vec<String> = Vec::new();
    for batch in batches {
        let mut worded_number = String::new();
        if batch != 0 {
            worded_number.push_str(&encode_0_999(batch));
            
            if current_unit >= 2 {
                worded_number.push_str(" ");
                worded_number.push_str(&units[current_unit - 2]);
            }

            worded_batches.push(worded_number);
        }
        current_unit -= 1;
    }

    worded_batches.join(" ")
}

fn encode_0_99() -> Vec<String> {
    vec![
        "zero".to_string(),
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),

        "ten".to_string(),
        "eleven".to_string(),
        "twelve".to_string(),
        "thirteen".to_string(),
        "fourteen".to_string(),
        "fifteen".to_string(),
        "sixteen".to_string(),
        "seventeen".to_string(),
        "eighteen".to_string(),
        "nineteen".to_string(),

        "twenty".to_string(),
        "twenty-one".to_string(),
        "twenty-two".to_string(),
        "twenty-three".to_string(),
        "twenty-four".to_string(),
        "twenty-five".to_string(),
        "twenty-six".to_string(),
        "twenty-seven".to_string(),
        "twenty-eight".to_string(),
        "twenty-nine".to_string(),

        "thirty".to_string(),
        "thirty-one".to_string(),
        "thirty-two".to_string(),
        "thirty-three".to_string(),
        "thirty-four".to_string(),
        "thirty-five".to_string(),
        "thirty-six".to_string(),
        "thirty-seven".to_string(),
        "thirty-eight".to_string(),
        "thirty-nine".to_string(),

        "forty".to_string(),
        "forty-one".to_string(),
        "forty-two".to_string(),
        "forty-three".to_string(),
        "forty-four".to_string(),
        "forty-five".to_string(),
        "forty-six".to_string(),
        "forty-seven".to_string(),
        "forty-eight".to_string(),
        "forty-nine".to_string(),

        "fifty".to_string(),
        "fifty-one".to_string(),
        "fifty-two".to_string(),
        "fifty-three".to_string(),
        "fifty-four".to_string(),
        "fifty-five".to_string(),
        "fifty-six".to_string(),
        "fifty-seven".to_string(),
        "fifty-eight".to_string(),
        "fifty-nine".to_string(),

        "sixty".to_string(),
        "sixty-one".to_string(),
        "sixty-two".to_string(),
        "sixty-three".to_string(),
        "sixty-four".to_string(),
        "sixty-five".to_string(),
        "sixty-six".to_string(),
        "sixty-seven".to_string(),
        "sixty-eight".to_string(),
        "sixty-nine".to_string(),

        "seventy".to_string(),
        "seventy-one".to_string(),
        "seventy-two".to_string(),
        "seventy-three".to_string(),
        "seventy-four".to_string(),
        "seventy-five".to_string(),
        "seventy-six".to_string(),
        "seventy-seven".to_string(),
        "seventy-eight".to_string(),
        "seventy-nine".to_string(),

        "eighty".to_string(),
        "eighty-one".to_string(),
        "eighty-two".to_string(),
        "eighty-three".to_string(),
        "eighty-four".to_string(),
        "eighty-five".to_string(),
        "eighty-six".to_string(),
        "eighty-seven".to_string(),
        "eighty-eight".to_string(),
        "eighty-nine".to_string(),

        "ninety".to_string(),
        "ninety-one".to_string(),
        "ninety-two".to_string(),
        "ninety-three".to_string(),
        "ninety-four".to_string(),
        "ninety-five".to_string(),
        "ninety-six".to_string(),
        "ninety-seven".to_string(),
        "ninety-eight".to_string(),
        "ninety-nine".to_string(),
    ]
}

fn encode_0_999(n: u64) -> String {
    if n > 999 {
        panic!("Call only with 0 to 999");
    }

    let words_0_99: Vec<String> = encode_0_99();

    if n < 100 {
        return words_0_99[n as usize].clone();
    }

    let hundreds_place = n / 100;
    let tens_and_units_place = n % 100;

    let mut worded_number = words_0_99[hundreds_place as usize].clone();
    worded_number.push_str(" hundred");

    if tens_and_units_place != 0 {
        worded_number.push_str(" ");
        worded_number.push_str(&words_0_99[tens_and_units_place as usize].clone());
    }

    worded_number
}

fn get_units() -> Vec<String> {
    vec![
        "thousand".to_string(),
        "million".to_string(),
        "billion".to_string(),
        "trillion".to_string(),
        "quadrillion".to_string(),
        "quintillion".to_string(),
    ]
}
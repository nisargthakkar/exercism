use std::cmp;

pub fn lsp(number_str: &'static str, window_size: usize) -> Result<u32, &'static str> {

    if window_size == 0 {
        return Ok(1);
    }

    if window_size > number_str.len() {
        return Err("Invalid window size");
    }

    let number_vec_char: Vec<char> = number_str.chars().collect();
    
    let mut max_product = 0;
    for window_char in number_vec_char.windows(window_size) {
        let mut window_digit: Vec<u32> = Vec::new();
        for c in window_char {
            match c.to_digit(10) {
                Some(n) => window_digit.push(n),
                None => return Err("Invalid number"),
            }
        }

        max_product = cmp::max(window_digit.iter().product(), max_product);

    }

    Ok(max_product)
}
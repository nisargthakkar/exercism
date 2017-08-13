pub fn is_valid(n: &'static str) -> bool {
    let number_vec_temp: Vec<&str> = n.trim().split_whitespace().collect();
    let number = number_vec_temp.join("");
    
    if number.len() <= 1 {
        return false;
    }

    if let Err(_) = number.parse::<u32>() {
        return false;
    }
    
    let number_vec_char: Vec<char> = number.chars().collect();

    let mut number_vec: Vec<u32> = number_vec_char.iter().map(|x| x.to_digit(10).unwrap()).collect();

    let mut sum: u32 = 0;

    for i in 0..number_vec.len() {
        if i%2 == number_vec.len()%2 {
            number_vec[i] *= 2;
            if number_vec[i] > 9 {
                number_vec[i] -= 9;
            }
        }

        sum += number_vec[i];
    }

    sum % 10 == 0
}
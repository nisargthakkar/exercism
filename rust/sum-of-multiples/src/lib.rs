pub fn sum_of_multiples(n: u32, factors: &Vec<u32>) -> u32 {
    let mut sum = 0;

    for i in 1..n {
        if is_multiple_of(i, factors) {
            sum += i;
        }
    }

    sum
}

fn is_multiple_of(n: u32, factors: &Vec<u32>) -> bool {
    for factor in factors {
        if n % factor == 0 {
            return true;
        }
    }

    false
}
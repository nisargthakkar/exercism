pub fn square_of_sum(n: u32) -> u32 {
    let sum_of_n = sum((1..n+1).collect());

    sum_of_n * sum_of_n
}

pub fn sum_of_squares(n: u32) -> u32 {
    sum((1..n+1).map(|x| x * x).collect())
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

fn sum(numbers: Vec<u32>) -> u32 {
    let mut sum = 0;
    for n in numbers {
        sum += n;
    }

    sum
}
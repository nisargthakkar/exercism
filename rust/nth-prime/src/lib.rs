pub fn nth(n: u32) -> Result<u32, &'static str> {
    if n == 0 {
        return Result::Err("Number must be greater than 0");
    }

    let mut n_iter = 0;
    let mut i = 1;
    while n_iter < n {
        i += 1;
        if is_prime(i) {
            n_iter += 1;
        }
    }

    Result::Ok(i)
}

fn is_prime (n: u32) -> bool {
    for i in 2..n/2+1 {
        if n % i == 0 {
            return false;
        }
    }

    true
}
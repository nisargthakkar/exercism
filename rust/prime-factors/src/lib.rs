pub fn factors(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    let mut prime_factors: Vec<u64> = Vec::new();

    let mut i = 2;

    let mut working_n = n;
    while i <= working_n {
        if i > (working_n as f64).sqrt() as u64 || is_prime(i, &primes) {
            primes.push(i);
            while working_n % i == 0 {
                prime_factors.push(i);
                working_n /= i;
            }
        }
        i += 1;
    }

    prime_factors
}

fn is_prime(n: u64, primes: &Vec<u64>) -> bool {
    for i in primes {
        if n % i == 0 {
            return false;
        }
    }

    true
}
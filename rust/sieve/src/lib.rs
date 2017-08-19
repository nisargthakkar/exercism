pub fn primes_up_to(n: u32) -> Vec<u32> {
    let working_vec: Vec<u32> = (2..n+1).collect();
    let mut count_vec: Vec<u32> = vec![0; (n-1) as usize];
    
    let mut prime_vec: Vec<u32> = Vec::new();

    let mut i: u32 = 0;
    while i < n-1 && n >= 2 {
        let prime = working_vec[i as usize];
        if count_vec[i as usize] == 0 {
            prime_vec.push(prime);
        }

        let mut j = i + 1;
        while j < n-1 {
            if (j + 2) % (i + 2) == 0 && count_vec[j as usize] == 0 {
                count_vec[j as usize] += 1;
            }

            j += 1;
        }

        i += 1;
    }
    
    prime_vec
}
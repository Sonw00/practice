pub fn nth(n: u32) -> u32 {
    let mut count = 2;
    let mut primes = Vec::<u32>::new();
    loop {
        if is_prime(count) {
            primes.push(count);
        }

        if primes.len() == (n + 1) as usize {
            return primes[n as usize];
        } else {
            count += 1;
        }
    }
}

fn is_prime(n: u32) -> bool {
    let mut divided_count = 0;
    for i in 1..=(n / 2) {
        if n % i == 0 {
            divided_count += 1;
        }
    }

    divided_count == 1
}

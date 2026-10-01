pub fn factors(mut n: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut candidate = 2;
    while n > 1 {
        while n.is_multiple_of(candidate) {
            primes.push(candidate);
            n /= candidate;
        }
        candidate += 1;
    }
    primes
}

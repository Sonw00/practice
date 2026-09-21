pub fn nth(n: u32) -> u32 {
    let mut element = 2;
    let mut count = 0;

    loop {
        if is_prime(element) {
            count += 1;
            if count == n + 1 {
                return element;
            }
        }

        element += 1;
    }
}

fn is_prime(n: u32) -> bool {
    for i in 2..n {
        if n.is_multiple_of(i) {
            return false;
        }
    }

    true
}

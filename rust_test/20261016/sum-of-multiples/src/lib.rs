pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|candidate| {
            factors
                .iter()
                .any(|factor| candidate.is_multiple_of(*factor))
        })
        .sum()
}

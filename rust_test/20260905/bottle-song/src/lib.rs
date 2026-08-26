pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut result = String::new();

    let numbers = [
        "no","one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];

    for i in (1..=start_bottles).rev() {
        let initial_bottle_count_word = numbers[(i) as usize];
        let take_down_word = numbers[(take_down) as usize];
        let remained_bottle_count_word = numbers[(i - take_down).max(0) as usize];

        result.push_str(&format!(
            "{} green bottles hanging on the wall,\n{} green bottles hanging on the wall,\nAnd if {} green bottle should accidentally fall,\nThere'll be {} green bottles hanging on the wall.",
            initial_bottle_count_word,
            initial_bottle_count_word,
            take_down_word,
            remained_bottle_count_word,
        ));

        if i > 1 {
            result.push_str("\n\n");
        }
    }

    result
}

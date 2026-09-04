pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut result = String::new();

    let numbers = [
        "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
    ];

    for i in ((start_bottles - take_down + 1)..=start_bottles).rev() {
        let initial_bottle_count_word = numbers[(i) as usize];
        let initial_bottle_word = if i == 1 {
            "bottle".to_string()
        } else {
            "bottles".to_string()
        };
        let remained_bottle_count_word = numbers[(i - 1) as usize].to_lowercase();
        let remained_bottle_word = if (i - 1) == 1 {
            "bottle".to_string()
        } else {
            "bottles".to_string()
        };

        result.push_str(&format!(
            "{} green {} hanging on the wall,\n{} green {} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {} green {} hanging on the wall.",
            initial_bottle_count_word,
            initial_bottle_word,
            initial_bottle_count_word,
            initial_bottle_word,
            remained_bottle_count_word,
            remained_bottle_word,
        ));

        if i > 1 {
            result.push_str("\n\n");
        }
    }

    result
}

pub fn raindrops(n: u32) -> String {

    if n.is_multiple_of(3) || n.is_multiple_of(5) || n.is_multiple_of(7) {
        let mut result: String = String::new();

        if n.is_multiple_of(3) {
            result.push_str("Pling");
        }

        if n.is_multiple_of(5) {
            result.push_str("Plang");
        }

        if n.is_multiple_of(7) {
            result.push_str("Plong");
        }
        
        result
    } else {
        n.to_string()
    }
}

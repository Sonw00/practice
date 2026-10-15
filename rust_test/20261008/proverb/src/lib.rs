pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = Vec::<String>::new();

    if !list.is_empty() {
        for i in 1..list.len(){
            proverb.push(format!("For want of a {} the {} was lost.",list[i-1], list[i]));
        }
        proverb.push(format!("And all for the want of a {}.", list[0]))
    }

    proverb.join("\n")
}

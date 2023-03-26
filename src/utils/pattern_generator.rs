pub fn generate_initial_pattern(word_len: String) -> String {
    let word_len_int = word_len.trim().parse::<i32>().unwrap();
    let mut pattern = String::new();
    for _ in 0..word_len_int {
        pattern.push('_');
    }
    let pattern_string = pattern.to_string();
    pattern_string
}
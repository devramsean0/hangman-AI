use std::collections::{HashSet, HashMap};

pub fn guess_word(words: Vec<String>, word_len: usize, pattern: &str, max_guesses: usize) -> Option<String> {
    let mut remaining_words: Vec<&String> = words
        .iter()
        .filter(|w| w.len() == word_len && w.chars().zip(pattern.chars()).all(|(wc, pc)| pc == '_' || wc == pc))
        .collect();
    let mut guessed_letters: HashSet<char> = HashSet::new();
    let mut current_best_guess: Option<char> = None;
    let mut current_best_remaining_words = remaining_words.clone();
    
    for _ in 0..max_guesses {
        let letter_frequencies = get_letter_frequencies(&remaining_words, &guessed_letters);
        let next_guess = get_next_guess(&letter_frequencies);
        guessed_letters.insert(next_guess);
        let num_occurrences = remaining_words.iter().fold(0, |acc, w| acc + w.matches(next_guess).count());
        
        if num_occurrences == 0 {
            remaining_words.retain(|w| !w.contains(next_guess));
        } else {
            let mut updated_remaining_words = Vec::new();
            for &w in &remaining_words {
                if w.contains(next_guess) {
                    updated_remaining_words.push(w);
                }
            }
            remaining_words = updated_remaining_words;
        }
        
        if remaining_words.is_empty() {
            return Some(pattern.chars().map(|c| if c == '_' { next_guess } else { c }).collect());
        } else if remaining_words.len() < current_best_remaining_words.len() {
            current_best_guess = Some(next_guess);
            current_best_remaining_words = remaining_words.clone();
        }
    }
    
    current_best_guess.map(|c| c.to_string())
}

fn get_letter_frequencies(words: &[&String], guessed_letters: &HashSet<char>) -> HashMap<char, usize> {
    let mut frequencies: HashMap<char, usize> = HashMap::new();
    for w in words {
        for c in w.chars() {
            if !guessed_letters.contains(&c) {
                *frequencies.entry(c).or_insert(0) += 1;
            }
        }
    }
    frequencies
}

fn get_next_guess(frequencies: &HashMap<char, usize>) -> char {
    frequencies
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(&c, _)| c)
        .unwrap_or('a') // arbitrary default value
}
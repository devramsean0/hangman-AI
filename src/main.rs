mod utils;
fn main() -> std::io::Result<()> {
    let mut continue_guessing = true;
    // Get words
    let words = utils::file_loader::read_words_from_file("words.txt").unwrap();
    // Get standard inputs
    let mut word_len = String::new();
    let mut max_guesses = 1;
    println!("Enter word length: ");
    std::io::stdin().read_line(&mut word_len)?;
    let word_len_usize = word_len.trim().parse::<usize>().unwrap();
    let mut max_guesses_usize = max_guesses.to_string().parse::<usize>().unwrap();
    // Compute pattern
    let pattern = utils::pattern_generator::generate_initial_pattern(word_len);
    println!("Initial Pattern: {:?}", pattern);
    let guess = utils::algorithm::guess_word(words, word_len_usize, pattern.as_str(), max_guesses_usize);
    println!("Initial Guess: {:?}", guess.unwrap());
    max_guesses= max_guesses+1;
    max_guesses_usize = max_guesses.to_string().parse::<usize>().unwrap();
    while continue_guessing {
        let words = utils::file_loader::read_words_from_file("words.txt").unwrap();
        let mut correct_guess = String::new();
        println!("Was the guess correct? (y/n): ");
        std::io::stdin().read_line(&mut correct_guess)?;
        if correct_guess.trim() == "y" {
            continue_guessing = false;
        } else {
            let mut pattern = String::new();
            println!("Enter new pattern (with correct characters from last guess in their respective places, and every other character filled with a `_`): ");
            std::io::stdin().read_line(&mut pattern)?; 
            let guess = utils::algorithm::guess_word(words.to_owned(), word_len_usize, pattern.as_str(), max_guesses_usize);
            println!("Guess: {:?}", guess.unwrap());
            max_guesses= max_guesses+1;
            max_guesses_usize = max_guesses.to_string().parse::<usize>().unwrap();
        }
    }
    Ok(())
}
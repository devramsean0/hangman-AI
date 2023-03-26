use std::io::*;
use std::fs::File;

pub fn read_words_from_file(file_path: &str)  -> std::io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut words = Vec::new();
    for line in reader.lines() {
        let line_unwrapped = line.unwrap().to_owned();
        words.push(line_unwrapped);
    }
    Ok(words)

}
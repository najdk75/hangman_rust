use super::difficulty::*;

pub struct Config {
    difficulty: Difficulty,
    word_to_guess: String,
    number_of_attempts: u8,
    current_word : String,
}


 impl Config {
    pub fn new(difficulty: Difficulty, word_to_guess: &str) -> Self {
        Self {
            difficulty,
            word_to_guess : word_to_guess.to_string(),
            number_of_attempts: 7,
            current_word : String::new(),
        }
    }
}
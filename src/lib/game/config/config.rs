use super::difficulty::*;

pub struct Config {
    difficulty: Difficulty,
    word_to_guess: String,
}

impl Config {
    pub fn new(difficulty: Difficulty, word_to_guess: String) -> Self {
        Self {
            difficulty,
            word_to_guess,
        }
    }
}

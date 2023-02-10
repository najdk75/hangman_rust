use crate::game::config::guess_processing::*;
use crate::menu_manager::user_input::*;
use std::collections::HashSet;

#[derive(Clone, Copy)]
pub enum GameState {
    Playing,
    Won,
    Lost,
}

pub struct Player {
    pub word_length: u8,
    pub current_word: Vec<char>,
    pub remaining_attempts: u8,
    pub letters_guessed_correctly: u8,
    pub game_state: GameState,
    pub previously_guessed: HashSet<String>,
}

impl Player {
    pub fn new(length_word_to_guess: usize, remaining_attempts: u8) -> Self {
        let empty_word = vec!['_'; length_word_to_guess];
        Self {
            word_length: length_word_to_guess as u8,
            current_word: empty_word,
            remaining_attempts,
            letters_guessed_correctly: 0,
            game_state: GameState::Playing,
            previously_guessed: HashSet::new(),
        }
    }

    pub fn display_word(&self) {
        for c in &self.current_word {
            print!("{} ", c);
        }
        println!();
    }

    pub fn game_status(&mut self) -> GameState {
        self.game_state
    }

    pub fn win(&mut self) {
        self.game_state = GameState::Won
    }

    pub fn lose(&mut self) {
        self.game_state = GameState::Lost
    }

    pub fn incorrect_guess(&mut self) {
        self.remaining_attempts -= 1;
        if self.remaining_attempts == 0 {
            self.lose()
        }
    }

    pub fn add_guessed_word(&mut self, input: &str) {
        self.previously_guessed.insert(input.to_string());
    }

    pub fn correct_guess(&mut self, correctly_guessed_letters: &u8) {
        self.letters_guessed_correctly += correctly_guessed_letters;
        if self.letters_guessed_correctly == self.word_length {
            self.win();
        }
    }

    pub fn process_guess(&mut self, word_to_guess: &str, user_input: &UserInput) -> bool {
        match user_input.is_letter {
            true => process_letter_guess(
                self,
                &user_input.input.chars().next().unwrap(),
                word_to_guess,
            ),
            false => process_word_guess(self, &user_input.input, word_to_guess),
        }
    }

    pub fn get_user_guess(&mut self) -> Option<UserInput> {
        let user_input = read_user_guess();

        if let false = self.previously_guessed.insert(user_input.input.clone()) {
            return None;
        };

        Some(user_input)
    }
}

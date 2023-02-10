use crate::game::config::difficulty::*;
use crate::game::run::hangman::*;
use crate::menu_manager::game_menu_option::*;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use rand::Rng;
use super::player::Player;

const ATTEMPTS: u8 = 7;

pub fn set_up() {
    display_main_menu();
    if let GameMenuOption::Quit = GameMenuOption::retrieve() {
        println!("See you later!");
        exit(0);
    }

    display_game_menu();
    let difficulty = Difficulty::retrieve();

    let word_to_guess = generate_random_word_by_difficulty(&difficulty);

    let mut player = Player::new(word_to_guess.len(), ATTEMPTS);

    hangman(&mut player, &word_to_guess);
}

pub fn load_words_to_guess() -> Vec<String> {
    let path = "words/words_to_guess.txt";
    let mut words_to_guess_file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not open the file: {}\t error : {}", path, e);
            exit(1);
        }
    };

    let mut words_to_guess = String::new();
    if let Err(e) = words_to_guess_file.read_to_string(&mut words_to_guess) {
        eprintln!("Error reading file:{}\t error: {}", path, e);
        exit(1);
    }

    words_to_guess
        .trim()
        .split('\n')
        .map(|word| word.to_owned())
        .collect()
}

pub fn generate_random_word_by_difficulty(difficulty: &Difficulty) -> String {
    let words_to_guess = load_words_to_guess();
    let length = words_to_guess.len();

    loop {
        let random_index = rand::thread_rng().gen_range(0, length);
        let word = words_to_guess[random_index].to_owned();
        let word_len = word.len();

        match difficulty {
            Difficulty::Easy => {
                if (0..6).contains(&word_len) {
                    return word;
                }
            }
            Difficulty::Medium => {
                if (6..=12).contains(&word_len) {
                    return word;
                }
            }
            Difficulty::Hard => {
                if word_len > 12 {
                    return word;
                }
            }
        }
    }
}

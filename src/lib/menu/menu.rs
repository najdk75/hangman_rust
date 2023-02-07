use super::user_input::{self, get_user_input};
use std::process::exit;

struct Config {
    difficulty: Difficulty,
    word_to_guess: String,
    number_of_attempts: u8,
}

impl Config {
    fn new(difficulty: Difficulty, word_to_guess: String) -> Self {
        Self {
            difficulty,
            word_to_guess,
            number_of_attempts: 7,
        }
    }
}
pub enum MenuChoice {
    Play,
    Exit,
}

impl MenuChoice {
    fn new(user_input: &str) -> Option<Self> {
        match user_input.to_lowercase().as_str() {
            "1" | "play" | "p" => Some(Self::Play),
            "2" | "exit" | "q" | "e" => Some(Self::Exit),
            _ => None,
        }
    }
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    fn new(user_input: &str) -> Option<Self> {
        match user_input {
            "1" => Some(Self::Easy),
            "2" => Some(Self::Hard),
            "3" => Some(Self::Hard),
            _ => None,
        }
    }
}

pub fn match_menu_choice() -> MenuChoice {
    let user_input = get_user_input();

    if let Err(e) = user_input {
        eprintln!("{}", e);
        exit(1);
    }
    let user_input = user_input.unwrap();

    match MenuChoice::new(&user_input) {
        Some(menu_choice) => menu_choice,
        None => {
            eprintln!("Difficulty does not exist.");
            exit(1);
        }
    }
}

pub fn match_difficulty() -> Difficulty {
    let user_input = get_user_input();

    if let Err(e) = user_input {
        eprintln!("{}", e);
        exit(1);
    }
    let user_input = user_input.unwrap();
    match Difficulty::new(&user_input) {
        Some(difficulty) => difficulty,
        None => {
            eprintln!("Difficulty does not exist.");
            exit(1);
        }
    }
}

pub fn main_menu() {
    println!("-- Hangman --");
    println!("1. Play");
    println!("2. Play");
}

pub fn game_menu() {
    println!("-- Hangman --");
    println!("Choose a difficulty:");
    println!("1. Easy");
    println!("2. Medium");
    println!("3. Hard");
}

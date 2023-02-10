use super::user_input::get_user_input;
use std::process::exit;

pub enum GameMenuOption {
    Play,
    Quit,
}

impl GameMenuOption {
    pub fn from_input(user_input: &str) -> Option<Self> {
        match user_input.to_lowercase().as_str() {
            "1" | "play" | "p" => Some(Self::Play),
            "2" | "quit" | "q" | "e" => Some(Self::Quit),
            _ => None,
        }
    }

    pub fn retrieve() -> GameMenuOption {
        let user_input = get_user_input().expect("Could not read chosen option.");
    
        match GameMenuOption::from_input(&user_input) {
            Some(menu_option) => menu_option,
            None => {
                eprintln!("Invalid menu option.");
                exit(1);
            }
        }
    }
}


pub fn display_main_menu() {
    println!("-- Hangman --");
    println!("1. Play");
    println!("2. Exit");
}

pub fn display_game_menu() {
    println!("-- Hangman --");
    println!("Choose a difficulty:");
    println!("1. Easy");
    println!("2. Medium");
    println!("3. Hard");
    println!()
}

pub fn display_end_alert(alert: &str, word_to_guess: &str) {
    println!("{} The word to guess was: {}.", alert, word_to_guess);
    println!()
}



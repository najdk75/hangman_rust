use crate::game::config::config::Config;
use crate::game::config::words::*;
use crate::menu::user_input::*;
use std::process::exit;

pub fn run(game_config: Config) {
    let word_to_guess = game_config.take_word();
    let length = word_to_guess.len() as u8;
    let word_to_guess_as_vec = word_as_vec(&word_to_guess);
    let mut current: Vec<String> = (0..length).map(|_| "_".to_owned()).collect();

    let mut attempts_left = game_config.attempts;
    let mut guessed: u8 = 0;

    loop {
        print_current(&current);
        if guessed == length {
            println!("Congratulations you have won");
            println!("{}", word_to_guess);
            exit(0); // peut etre revenir au menu un jour..
        } else if attempts_left == 0 {
            println!("You lost!");
            println!("The word to guess was : {}", word_to_guess);
            exit(0);
        } else {
            println!("Choose a letter");
            let letter = get_letter();
            if word_to_guess.contains(&letter) {
                guessed += 1;
                let indexes = get_letter_occurrences(&word_to_guess_as_vec, &letter);
                current = replace_letters(&current, &indexes, &letter)
            } else {
                attempts_left -= 1;
            }
        }
    }
}

fn print_current(current: &[String]) {
    for letter in current {
        print!("{} ", letter);
    }
}

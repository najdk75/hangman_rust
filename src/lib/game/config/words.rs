use super::difficulty::*;
use rand::{random, Rng};
use std::fs::File;
use std::io::Read;
use std::process::exit;

fn generate_words() -> Vec<String> {
    let path = "words/words_to_guess.txt";
    let mut words_to_guess_file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not open the file {}", path);
            exit(1);
        }
    };

    let mut words_to_guess = String::new();
    if let Err(e) = words_to_guess_file.read_to_string(&mut words_to_guess) {
        eprintln!("Error reading file:{} \t error: {}", path, e);
        exit(1);
    }

    let words_to_guess_vec: Vec<String> = words_to_guess
        .trim()
        .split("\n")
        .map(|word| word.to_owned())
        .collect();

    println!("{}", words_to_guess_vec.len());
    words_to_guess_vec
}

pub fn generate_random_word(difficulty: &Difficulty) -> String {
    let words_to_guess = generate_words();

    let words_to_guess: Vec<String> = match difficulty {
        Difficulty::Easy => words_to_guess
            .iter()
            .filter_map(|s| {
                if s.len() < 4 {
                    Some(s.to_owned())
                } else {
                    None
                }
            })
            .collect(),

        Difficulty::Medium => words_to_guess
            .iter()
            .filter_map(|s| {
                if s.len() >= 4 && s.len() <= 10 {
                    Some(s.to_owned())
                } else {
                    None
                }
            })
            .collect(),

        Difficulty::Hard => words_to_guess
            .iter()
            .filter_map(|s| {
                if s.len() > 15 {
                    Some(s.to_owned())
                } else {
                    None
                }
            })
            .collect(),
    };

    let length = words_to_guess.len();
    println!("Length {}", length);
    let random_index = rand::thread_rng().gen_range(0, length);

    words_to_guess
        .get(random_index)
        .map(String::to_owned)
        .unwrap_or_else(|| {
            eprintln!("Could not generate a random word");
            exit(1);
        })
}

pub fn word_as_vec(word: &str) -> Vec<String> {
    word.chars().map(|c| c.to_string()).collect()
}

pub fn get_letter_occurrences(letters: &[String], letter: &str) -> Vec<usize> {
    let mut indexes: Vec<usize> = Vec::new();
    for (index, l) in letters.iter().enumerate() {
        if l == letter {
            indexes.push(index);
        }
    }

    indexes
}

pub fn replace_letters(letters: &[String], occurences: &[usize], letter: &str) -> Vec<String> {
    let mut new_letters: Vec<String> = letters.to_vec();

    for o in occurences {
        new_letters[*o] = letter.to_string();
    }

    new_letters
}

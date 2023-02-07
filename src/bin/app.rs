use hangman::game::{
    config::{self, difficulty::*, words::*},
    run,
};
fn main() {
    let word = generate_random_word(&Difficulty::Hard);

    println!("{}", word);
}

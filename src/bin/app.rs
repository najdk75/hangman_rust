use hangman::game::{
    config::{self, words::*},
    run,
};
fn main() {
    let word = generate_random_word();

    println!("{}", word);
}

use hangman::game::{config::{words::*, self}, run};
fn main() {
 let word = generate_random_word();

 println!("{}",word);
}

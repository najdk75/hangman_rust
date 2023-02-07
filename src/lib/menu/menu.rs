use super::user_input;



enum Menu {
    Play,
    Exit,
}

impl Menu {

    fn new(user_input : &str) -> Option<Self> {
        match user_input {
            "1" => Some(Self::Play),
            "2" => Some(Self::Exit),
            _ => None
        }
    }

}

enum Difficulty {
    Easy,
    Medium,
    Hard
}

impl Difficulty {

    fn new(user_input : &str) -> Option<Self> {
        match user_input {
            "1" => Some(Self::Easy),
            "2" => Some(Self::Hard),
            "3" => Some(Self::Hard),
            _ => None
        }
    }

}



pub fn main_menu() {
    println!("-- Hangman --");
    println!("1. Play");
    println!("2. Play");

}


pub fn game_menu(){
    println!("-- Hangman --");
    println!("Choose a difficulty:");
    println!("1. Easy");
    println!("2. Medium");
    println!("3. Hard");
}
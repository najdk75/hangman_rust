use crate::menu_manager::user_input::get_user_input;
use std::process::exit;

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn new(user_input: &str) -> Option<Self> {
        match user_input {
            "1" => Some(Self::Easy),
            "2" => Some(Self::Hard),
            "3" => Some(Self::Hard),
            _ => None,
        }
    }

    pub fn retrieve() -> Difficulty {
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
    


}


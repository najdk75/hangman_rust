use super::user_input::get_user_input;
use std::process::exit;

pub enum MenuChoice {
    Play,
    Exit,
}

impl MenuChoice {
    pub fn new(user_input: &str) -> Option<Self> {
        match user_input.to_lowercase().as_str() {
            "1" | "play" | "p" => Some(Self::Play),
            "2" | "exit" | "q" | "e" => Some(Self::Exit),
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

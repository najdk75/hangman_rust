use std::io::{self, Error};

pub struct UserInput {
    pub input: String,
    pub is_letter: bool,
}

impl UserInput {
    pub fn new(input: &str) -> Self {
        let input = input.to_lowercase();
        let is_letter = input.len() == 1
            && input
                .chars()
                .next()
                .map_or(false, |letter| letter.is_alphabetic());

        Self { input, is_letter }
    }
}

pub fn get_user_input() -> Result<String, Error> {
    let mut user_input = String::from("");
    loop {
        io::stdin().read_line(&mut user_input)?;
        if user_input.trim().is_empty() {
            println!("Please, do not type an empty character");
        } else {
            break;
        }
    }

    Ok(user_input.trim().to_owned())
}

pub fn read_user_guess() -> UserInput {

    let user_guess = get_user_input().expect("Could not read user's guess.");
    UserInput::new(&user_guess)
}

use std::io::{self, Error};
use std::process::exit;

pub fn get_user_input() -> Result<String, Error> {
    let mut buffer = String::from("");

    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}


pub fn get_letter() -> String {
    let letter = get_user_input();
    if let Err(e) = letter {
        eprintln!("Could not get user_input");
        exit(1);
    }
    let letter = letter.unwrap().to_lowercase().as_str();
    

    if !(letter >= "a" && letter <= "z") {
        eprintln!("Please choose a letter between 'a' and 'z'")
        exit(1);
    }
    letter.to_owned()
}
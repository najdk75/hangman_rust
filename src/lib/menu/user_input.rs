use std::io::{self, Error};

pub fn get_user_input() -> Result<String,Error> {

    let mut buffer = String::from("");

    io::stdin().read_line(&mut buffer)?;
    
    Ok(buffer.trim().to_owned())

}
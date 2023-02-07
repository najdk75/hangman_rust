use hangman::menu::{menu,user_input};


fn main() {


    menu::main_menu();
    menu::game_menu();

    let m = user_input::get_user_input();
    match m {
        Ok(s) => println!("{}",s),
        Err(e) => println!("{}",e),
    }

}


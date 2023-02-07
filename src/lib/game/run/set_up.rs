use crate::game::config::config::*;
use crate::game::config::difficulty::*;
use crate::game::run::hangman::*;
use crate::menu::menu::*;
use crate::menu::menu_choice::*;
use std::process::exit;

pub fn hangman() {
    main_menu();
    if let MenuChoice::Exit = match_menu_choice() {
        println!("See you later!");
        exit(0);
    }

    game_menu();
    let difficulty = match_difficulty();

    // TODO : générer un mot aléatoire

    let game_config = Config::new(difficulty, "Hello");

    run(game_config);
}

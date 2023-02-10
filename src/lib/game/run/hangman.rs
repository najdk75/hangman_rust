use crate::game::config::game_setup::*;
use crate::game::config::player::*;
use crate::menu_manager::game_menu_option::*;

pub fn hangman(player: &mut Player, word_to_guess: &str) {
    let mut end_game_alert = String::new();

    loop {
        if let GameState::Lost | GameState::Won = player.game_status() {
            break;
        }

        player.display_word();
        println!("Choose a letter, or guess the word directly!");

        let player_guess = player.get_user_guess();
        if player_guess.is_none() {
            println!("You already typed that!");
            continue;
        }
        match player.process_guess(word_to_guess, &player_guess.unwrap()) {
            true => println!("Good job!"),
            false => println!(
                "Wrong answer..., Attempts left : {}",
                player.remaining_attempts
            ),
        }
    }

    match player.game_status() {
        GameState::Won => end_game_alert.push_str("Congratulations, you have won."),
        GameState::Lost => end_game_alert.push_str("You have lost."),
        _ => {}
    }

    display_end_alert(&end_game_alert, word_to_guess);
    set_up();
}

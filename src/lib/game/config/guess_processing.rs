use super::player::*;


pub fn process_letter_guess(player: &mut Player, letter: &char, word_to_guess: &str) -> bool {
    let letter_indexes = find_letter_occurrences(word_to_guess, letter);

    match letter_indexes.is_empty() {
        true => {
            player.incorrect_guess();
            false
        }
        false => {
            reveal_matching_letters(player, letter, &letter_indexes);
            true
        }
    }
}

pub fn find_letter_occurrences(letters: &str, letter: &char) -> Vec<usize> {
    let mut indexes: Vec<usize> = Vec::new();
    for (index, l) in letters.chars().enumerate() {
        if l == *letter {
            indexes.push(index);
        }
    }
    indexes
}

pub fn reveal_matching_letters(player: &mut Player, letter: &char, indexes: &[usize]) {
    for i in indexes {
        player.current_word[*i] = *letter;
    }
    let has_guessed = indexes.len() as u8;
    player.correct_guess(&has_guessed);
}

pub fn process_word_guess(player: &mut Player, player_input: &str, word_to_guess: &str) -> bool {
    if player_input == word_to_guess {
        player.win();
        true
    } else {
        player.incorrect_guess();
        false
    }
}

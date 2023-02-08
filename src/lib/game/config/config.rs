pub struct Config {
    //difficulty: Difficulty,
    word_to_guess: String,
    pub attempts: u8,
}

impl Config {
    pub fn new(word_to_guess: String) -> Self {
        Self {
            //difficulty,
            word_to_guess,
            attempts: 7,
        }
    }

    pub fn take_word(&self) -> String {
        self.word_to_guess.to_owned()
    }
}

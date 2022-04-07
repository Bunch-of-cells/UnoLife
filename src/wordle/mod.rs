pub mod game;
pub use game::*;
pub mod ui;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_was_not_in_list() {
        let mut wordle = Game::new();
        assert_eq!(
            wordle.guess("ILuvU".to_string()),
            Err(GuessError::WordWasNotInList)
        );
    }

    #[test]
    fn correct_guess() {
        let mut wordle = Game::new();
        assert_eq!(
            wordle.guess(wordle.word.to_string()),
            Ok(GuessResult::Right)
        );
    }
}

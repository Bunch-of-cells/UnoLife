pub mod game;
pub use game::*;
pub mod ui;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wordle_instance() {
        let mut wordle = Game::new();
        assert_eq!(
            wordle.guess("ILuvU".to_string()),
            Err(GuessError::WordWasNotInList)
        );
    }
}

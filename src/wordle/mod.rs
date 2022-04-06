#![allow(dead_code)]

pub mod game;
pub use game::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn wordle_instance() {
        let mut wordle = Game::new();
        assert_eq!(wordle.guess("above".to_string()), GuessResult::Wrong);
    }
}

use lazy_static::lazy_static;
use rand::prelude::IteratorRandom;
use std::{error::Error, fmt::Display, fs, iter::Flatten};

const GUESSES: usize = 6;
lazy_static! {
    static ref WORDS: Vec<String> = {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let words = fs::read_to_string(assets.join("wordle_list.txt")).unwrap();
        words.split_whitespace().map(|a| a.to_lowercase()).collect()
    };
}

pub struct Game {
    word: &'static str,
    guesses: [Option<Guess>; GUESSES],
    tries: usize,
}

impl Game {
    pub fn new() -> Self {
        let word = WORDS.iter().choose(&mut rand::thread_rng()).unwrap();
        Self {
            word,
            guesses: [0; GUESSES].map(|_| None),
            tries: 0,
        }
    }

    pub fn guess(&mut self, guess: String) -> Result<GuessResult, GuessError> {
        if guess.len() != 5 {
            return Err(GuessError::NotLongEnough);
        }
        if self.tries >= GUESSES {
            return Err(GuessError::NotLongEnough);
        }
        if guess.chars().any(|c| !c.is_alphabetic()) || !WORDS.contains(&guess) {
            return Err(GuessError::WordWasNotInList);
        }
        if guess == self.word {
            return Ok(GuessResult::Right);
        }
        self.guesses[self.tries] = Some(Guess::new(guess, self.word));
        self.tries += 1;
        if self.tries >= GUESSES {
            return Ok(GuessResult::GameOver(self.word));
        }
        Ok(GuessResult::Wrong)
    }

    pub fn guesses(&self) -> Flatten<std::slice::Iter<'_, Option<Guess>>> {
        self.guesses.iter().flatten()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GuessError {
    WordWasNotInList,
    NotLongEnough,
}

impl Display for GuessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuessError::WordWasNotInList => write!(f, "Word was not in list"),
            GuessError::NotLongEnough => write!(f, "Word was not long enough"),
        }
    }
}

impl Error for GuessError {}

#[derive(Debug, Clone, PartialEq)]
pub enum GuessResult<'a> {
    Right,
    Wrong,
    GameOver(&'a str),
}

#[derive(Debug, Clone)]
pub struct Guess {
    guess: String,
    word: &'static str,
}

impl PartialEq for Guess {
    fn eq(&self, other: &Self) -> bool {
        self.guess == other.guess
    }
}

impl Guess {
    fn new(guess: String, word: &'static str) -> Self {
        Self { guess, word }
    }

    pub fn guess(&self) -> &str {
        &self.guess
    }

    pub fn result(&self) -> [CharGuess; 5] {
        let mut array = [CharGuess::Incorrect; 5];
        let mut outta_order = 0;
        for (i, (guessed, correct)) in self.guess.chars().zip(self.word.chars()).enumerate() {
            if guessed == correct {
                array[i] = CharGuess::Correct;
            } else if self.word.chars().filter(|&a| a == guessed).count() - outta_order > 0 {
                outta_order += 1;
                array[i] = CharGuess::OutOfOrder;
            }
        }
        array
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CharGuess {
    Correct,
    Incorrect,
    OutOfOrder,
}

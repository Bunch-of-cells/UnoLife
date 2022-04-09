use lazy_static::lazy_static;
use rand::prelude::IteratorRandom;
use std::{collections::{HashMap, hash_map::Entry}, error::Error, fmt::Display, fs};

const GUESSES: usize = 6;
lazy_static! {
    static ref WORDS: Vec<String> = {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let words = fs::read_to_string(assets.join("wordle_list.txt")).unwrap();
        words.split_whitespace().map(|a| a.to_uppercase()).collect()
    };
}

pub struct Game {
    pub word: &'static str,
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

    pub fn guess(&mut self, guess: &String) -> Result<GuessResult, GuessError> {
        if guess.len() != 5 {
            return Err(GuessError::NotLongEnough);
        }
        if self.tries >= GUESSES {
            return Err(GuessError::NotLongEnough);
        }
        if guess.chars().any(|c| !c.is_alphabetic()) || !WORDS.contains(guess) {
            return Err(GuessError::WordWasNotInList);
        }
        let correct = guess == self.word;
        self.guesses[self.tries] = Some(Guess::new(guess, self.word));
        if correct {
            return Ok(GuessResult::Right);
        }
        self.tries += 1;
        if self.tries >= GUESSES {
            return Err(GuessError::GameOver(self.word));
        }
        Ok(GuessResult::Wrong)
    }

    pub fn guesses(&self) -> &[Option<Guess>] {
        &self.guesses
    }

    pub fn reset(&mut self) {
        self.tries = 0;
        self.guesses = [0; GUESSES].map(|_| None);
        self.word = WORDS.iter().choose(&mut rand::thread_rng()).unwrap();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GuessError<'a> {
    WordWasNotInList,
    NotLongEnough,
    GameOver(&'a str),
}

impl Display for GuessError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuessError::WordWasNotInList => write!(f, "Word was not in list"),
            GuessError::NotLongEnough => write!(f, "Word was not long enough"),
            GuessError::GameOver(word) => write!(f, "Game over, word was {}", word),
        }
    }
}

impl Error for GuessError<'_> {}

#[derive(Debug, Clone, PartialEq)]
pub enum GuessResult {
    Right,
    Wrong,
}

#[derive(Debug, Clone)]
pub struct Guess {
    result: [CharGuess; 5],
}

impl Guess {
    fn new(guess: &str, word: &'static str) -> Self {
        let mut seen = HashMap::<char, Vec<(usize, bool)>>::new();
        let mut array = [CharGuess {
            char: ' ',
            type_: GuessType::Incorrect,
        }; 5];
        for (i, (guessed, correct)) in guess.chars().zip(word.chars()).enumerate() {
            if let Entry::Vacant(e) = seen.entry(guessed) {
                e.insert(Vec::new());
                seen.get_mut(&guessed).unwrap();
            }
            let outta = seen.get_mut(&guessed).unwrap();
            if guessed == correct {
                if word.chars().filter(|&a| a == guessed).count() - outta.len() == 0 {
                    outta.remove(outta.iter().position(|a| !a.1).unwrap());
                }
                outta.push((i, true));
                array[i] = CharGuess {
                    char: guessed,
                    type_: GuessType::Correct,
                };
            } else if word.chars().filter(|&a| a == guessed).count() - outta.len() > 0 {
                outta.push((i, false));
                array[i] = CharGuess {
                    char: guessed,
                    type_: GuessType::OutOfOrder,
                };
            } else {
                array[i] = CharGuess {
                    char: guessed,
                    type_: GuessType::Incorrect,
                };
            }
        }
        Self { result: array }
    }

    pub fn result(&self) -> &[CharGuess] {
        &self.result
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CharGuess {
    pub char: char,
    pub type_: GuessType,
}

#[derive(Debug, Clone, Copy)]
pub enum GuessType {
    Correct,
    Incorrect,
    OutOfOrder,
}

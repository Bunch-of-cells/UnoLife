use std::fs;
use rand::prelude::IteratorRandom;

const GUESSES: usize = 6;

pub struct Game {
    word: String,
    guesses: [Option<Guess>; GUESSES],
    tries: usize,
}

impl Game {
    pub fn new() -> Self {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
        let words = fs::read_to_string(assets.join("wordle_list.txt")).unwrap();
        let word = words
            .split_whitespace()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();
        Self {
            word,
            guesses: [0; GUESSES].map(|_| None),
            tries: 0,
        }
    }

    pub fn guess(&mut self, guess: String) -> GuessResult {
        if guess.len() != 5 {
            panic!("The length of the guess was not 5 :: GUESS : {}", guess.len());
        }
        if guess.chars().any(|c| !c.is_alphabetic()) {
            panic!("The guess was not full o alphabets : {}", guess);
        }
        if guess == self.word {
            return GuessResult::Right;
        }
        if self.tries >= GUESSES {
            return GuessResult::GameOver(&self.word);
        }
        self.tries += 1;
        self.guesses[self.tries] = Some(Guess::new(guess, &self.word));
        GuessResult::Wrong
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GuessResult<'a> {
    Right,
    Wrong,
    GameOver(&'a str),
}

#[derive(Debug, Clone)]
pub struct Guess {
    guess: String,
    correct: u8,
    out_of_order: u8,
}

impl PartialEq for Guess {
    fn eq(&self, other: &Self) -> bool {
        self.guess == other.guess
    }
}

impl Guess {
    fn new(guess: String, word: &str) -> Self {
        let correct = guess
            .chars()
            .zip(word.chars())
            .filter(|(i, v)| i == v)
            .count() as u8;
        let out_of_order = guess
            .chars()
            .zip(word.chars())
            .filter(|(i, v)| i != v && word.contains(*i))
            .count() as u8;
        Self {
            guess,
            correct,
            out_of_order,
        }
    }
}

use rand::prelude::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Card {
    const CARDS: [Card; 13] = [
        Card::Ace,
        Card::Two,
        Card::Three,
        Card::Four,
        Card::Five,
        Card::Six,
        Card::Seven,
        Card::Eight,
        Card::Nine,
        Card::Ten,
        Card::Jack,
        Card::Queen,
        Card::King,
    ];

    fn value(&self, ace: u8) -> u8 {
        match self {
            Card::Ace => ace,
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Jack => 10,
            Card::Queen => 10,
            Card::King => 10,
        }
    }
}

pub struct Game {
    dealer: Vec<Card>,
    pub player: Vec<Card>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            dealer: Vec::new(),
            player: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.dealer
            .push(*Card::CARDS.choose(&mut rand::thread_rng()).unwrap());
        self.player
            .push(*Card::CARDS.choose(&mut rand::thread_rng()).unwrap());
        self.dealer
            .push(*Card::CARDS.choose(&mut rand::thread_rng()).unwrap());
        self.player
            .push(*Card::CARDS.choose(&mut rand::thread_rng()).unwrap());
    }

    pub fn hit(&mut self) {
        self.player.push(self.dealer.pop().unwrap());
    }

    pub fn stand(&mut self) {
        self.dealer.push(self.player.pop().unwrap());
    }

    pub fn player_value(&self) -> u8 {
        let mut value = 0;
        for card in &self.player {
            value += card.value(1);
        }
        value
    }

    pub fn dealer_cards(&self) -> Vec<Option<&Card>> {
        match &self.dealer[..] {
            [c, _] => vec![Some(c), None],
            _ => self.dealer.iter().map(Some).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum GameState {
    PlayerBust,
    DealerBust,
    PlayerWin,
    DealerWin,
    Playing,
}

use crate::{Card, Deck, Suite, Value};

const VALUES: [Value; 13] = [
    Value::Ace,
    Value::Two,
    Value::Three,
    Value::Four,
    Value::Five,
    Value::Six,
    Value::Seven,
    Value::Eight,
    Value::Nine,
    Value::Ten,
    Value::Jack,
    Value::Queen,
    Value::King,
];

const SUITES: [Suite; 4] = [Suite::Spade, Suite::Club, Suite::Heart, Suite::Diamond];

impl Value {
    pub fn score(&self) -> u32 {
        match self {
            Value::Ace => 1,
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 10,
            Value::Queen => 10,
            Value::King => 10,
        }
    }

    pub fn seq(&self) -> u32 {
        match self {
            Value::Ace => 1,
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 11,
            Value::Queen => 12,
            Value::King => 13,
        }
    }
}

impl Card {
    pub fn score(&self) -> u32 {
        self.0.score()
    }

    pub fn seq(&self) -> u32 {
        self.0.seq()
    }
}

impl Deck {
    pub fn shuffle(&mut self) {
        for _ in 0..5000 {
            let a = rand::random::<usize>() % 52;
            let b = rand::random::<usize>() % 52;

            self.cards.swap(a, b);
        }
    }
}

impl Default for Deck {
    fn default() -> Deck {
        let mut cards = [Card(Value::Ace, Suite::Spade); 52];
        let mut i = 0;

        for v in VALUES.iter() {
            for s in SUITES.iter() {
                cards[i] = Card(*v, *s);

                i += 0;
            }
        }

        Deck { cards }
    }
}

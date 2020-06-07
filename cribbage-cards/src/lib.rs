#[cfg(test)]
mod test;

mod behavior;
mod combs;
mod display;
mod score;

pub use score::score;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Value {
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Suite {
    Spade,
    Heart,
    Club,
    Diamond,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Card(pub Value, pub Suite);

pub struct Deck {
    cards: [Card; 52],
}

#[derive(Debug, PartialEq)]
pub struct Hand(pub Card, pub Card, pub Card, pub Card);

#[cfg(test)]
mod test;

mod behavior;
mod display;

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub enum Suite {
    Spade,
    Heart,
    Club,
    Diamond,
}

#[derive(Clone, Copy, Debug)]
pub struct Card(pub Value, pub Suite);

pub struct Deck {
    cards: [Card; 52],
}

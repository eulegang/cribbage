use crate::{Card, Suite, Value};
use std::fmt::{Display, Formatter, Result};

impl Display for Suite {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        let chr = match self {
            Suite::Spade => '\u{2660}',
            Suite::Heart => '\u{2665}',
            Suite::Club => '\u{2663}',
            Suite::Diamond => '\u{2666}',
        };

        write!(fmt, "{}", chr)
    }
}

impl Display for Value {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        let repr = match self {
            Value::Ace => "A",
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4",
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine => "9",
            Value::Ten => "10",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::King => "K",
        };

        write!(fmt, "{}", repr)
    }
}

impl Display for Card {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        let Card(value, suite) = self;
        write!(fmt, "{}{}", value, suite)
    }
}

use cribbage_cards::{Card, Suite, Value};

macro_rules! assert_format {
    ($input: expr, $output: expr) => {
        assert_eq!(format!("{}", $input), $output)
    };
}

#[test]
fn display_suite() {
    assert_format!(Suite::Spade, "♠");
    assert_format!(Suite::Heart, "♥");
    assert_format!(Suite::Club, "♣");
    assert_format!(Suite::Diamond, "♦");
}

#[test]
fn display_value() {
    assert_format!(Value::Ace, "A");
    assert_format!(Value::Two, "2");
    assert_format!(Value::Three, "3");
    assert_format!(Value::Four, "4");
    assert_format!(Value::Five, "5");
    assert_format!(Value::Six, "6");
    assert_format!(Value::Seven, "7");
    assert_format!(Value::Eight, "8");
    assert_format!(Value::Nine, "9");
    assert_format!(Value::Ten, "10");
    assert_format!(Value::Jack, "J");
    assert_format!(Value::Queen, "Q");
    assert_format!(Value::King, "K");
}

#[test]
fn display_card() {
    assert_format!(Card(Value::Ace, Suite::Spade), "A♠");
    assert_format!(Card(Value::King, Suite::Heart), "K♥");
}

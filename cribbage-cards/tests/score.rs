use cribbage_cards::{score, Card, Hand, Suite, Value};

#[test]
fn max_score() {
    let s = score(
        &Hand(
            Card(Value::Five, Suite::Spade),
            Card(Value::Five, Suite::Club),
            Card(Value::Five, Suite::Diamond),
            Card(Value::Jack, Suite::Heart),
        ),
        Card(Value::Five, Suite::Heart),
    );

    assert_eq!(s, 29);
}

#[test]
fn single_pair() {
    let s = score(
        &Hand(
            Card(Value::Five, Suite::Spade),
            Card(Value::Five, Suite::Club),
            Card(Value::Six, Suite::Diamond),
            Card(Value::Two, Suite::Heart),
        ),
        Card(Value::Ace, Suite::Heart),
    );

    assert_eq!(s, 2);
}

#[test]
fn triplet() {
    let s = score(
        &Hand(
            Card(Value::Six, Suite::Spade),
            Card(Value::Six, Suite::Club),
            Card(Value::Six, Suite::Diamond),
            Card(Value::Two, Suite::Heart),
        ),
        Card(Value::Four, Suite::Heart),
    );

    assert_eq!(s, 6);
}

#[test]
fn quadruplet() {
    let s = score(
        &Hand(
            Card(Value::Six, Suite::Spade),
            Card(Value::Six, Suite::Club),
            Card(Value::Six, Suite::Diamond),
            Card(Value::Six, Suite::Heart),
        ),
        Card(Value::Four, Suite::Heart),
    );

    assert_eq!(s, 12);
}

#[test]
fn double_pairs() {
    let s = score(
        &Hand(
            Card(Value::Six, Suite::Spade),
            Card(Value::Six, Suite::Club),
            Card(Value::Four, Suite::Diamond),
            Card(Value::Eight, Suite::Heart),
        ),
        Card(Value::Four, Suite::Heart),
    );

    assert_eq!(s, 4);
}

#[test]
fn fifteens() {
    let s = score(
        &Hand(
            Card(Value::Five, Suite::Spade),
            Card(Value::Jack, Suite::Club),
            Card(Value::Ten, Suite::Diamond),
            Card(Value::King, Suite::Heart),
        ),
        Card(Value::Ace, Suite::Heart),
    );

    assert_eq!(s, 6);
}

#[test]
fn single_run() {
    let s = score(
        &Hand(
            Card(Value::Jack, Suite::Spade),
            Card(Value::Queen, Suite::Club),
            Card(Value::King, Suite::Diamond),
            Card(Value::Two, Suite::Heart),
        ),
        Card(Value::Ace, Suite::Heart),
    );

    assert_eq!(s, 3);
}

#[test]
fn double_run() {
    let s = score(
        &Hand(
            Card(Value::Jack, Suite::Spade),
            Card(Value::Queen, Suite::Club),
            Card(Value::King, Suite::Diamond),
            Card(Value::King, Suite::Heart),
        ),
        Card(Value::Ace, Suite::Heart),
    );

    assert_eq!(s, 8);
}

#[test]
fn triple_run() {
    let s = score(
        &Hand(
            Card(Value::Jack, Suite::Spade),
            Card(Value::Queen, Suite::Club),
            Card(Value::King, Suite::Diamond),
            Card(Value::King, Suite::Heart),
        ),
        Card(Value::King, Suite::Heart),
    );

    assert_eq!(s, 15);
}

#[test]
fn double_double_run() {
    let s = score(
        &Hand(
            Card(Value::Jack, Suite::Spade),
            Card(Value::Queen, Suite::Club),
            Card(Value::King, Suite::Diamond),
            Card(Value::Jack, Suite::Diamond),
        ),
        Card(Value::King, Suite::Heart),
    );

    assert_eq!(s, 16);
}

#[test]
fn despiration() {
    let s = score(
        &Hand(
            Card(Value::Two, Suite::Spade),
            Card(Value::Four, Suite::Spade),
            Card(Value::Six, Suite::Spade),
            Card(Value::Eight, Suite::Spade),
        ),
        Card(Value::Ten, Suite::Heart),
    );

    assert_eq!(s, 4);
}

#[test]
fn despiration_with_cut() {
    let s = score(
        &Hand(
            Card(Value::Two, Suite::Spade),
            Card(Value::Four, Suite::Spade),
            Card(Value::Six, Suite::Spade),
            Card(Value::Eight, Suite::Spade),
        ),
        Card(Value::Ten, Suite::Spade),
    );

    assert_eq!(s, 5);
}

#[test]
fn sample_hands() {
    let s = score(
        &Hand(
            Card(Value::Four, Suite::Heart),
            Card(Value::Five, Suite::Spade),
            Card(Value::Six, Suite::Club),
            Card(Value::Six, Suite::Diamond),
        ),
        Card(Value::Four, Suite::Spade),
    );

    assert_eq!(s, 24);
}

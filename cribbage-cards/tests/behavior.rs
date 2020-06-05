use cribbage_cards::Value;

#[test]
fn test_score() {
    assert_eq!(Value::Ace.score(), 1);
    assert_eq!(Value::Two.score(), 2);
    assert_eq!(Value::Three.score(), 3);
    assert_eq!(Value::Four.score(), 4);
    assert_eq!(Value::Five.score(), 5);
    assert_eq!(Value::Six.score(), 6);
    assert_eq!(Value::Seven.score(), 7);
    assert_eq!(Value::Eight.score(), 8);
    assert_eq!(Value::Nine.score(), 9);
    assert_eq!(Value::Ten.score(), 10);
    assert_eq!(Value::Jack.score(), 10);
    assert_eq!(Value::Queen.score(), 10);
    assert_eq!(Value::King.score(), 10);
}

#[test]
fn test_seq() {
    assert_eq!(Value::Ace.seq(), 1);
    assert_eq!(Value::Two.seq(), 2);
    assert_eq!(Value::Three.seq(), 3);
    assert_eq!(Value::Four.seq(), 4);
    assert_eq!(Value::Five.seq(), 5);
    assert_eq!(Value::Six.seq(), 6);
    assert_eq!(Value::Seven.seq(), 7);
    assert_eq!(Value::Eight.seq(), 8);
    assert_eq!(Value::Nine.seq(), 9);
    assert_eq!(Value::Ten.seq(), 10);
    assert_eq!(Value::Jack.seq(), 11);
    assert_eq!(Value::Queen.seq(), 12);
    assert_eq!(Value::King.seq(), 13);
}

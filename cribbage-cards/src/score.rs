use crate::combs::CombExt;
use crate::{Card, Hand, Value};

pub fn score(hand: &Hand, cut: Card) -> u32 {
    let mut score = 0;

    let pairs = pairs(hand, cut);
    score += pairs;

    let fifteens = fifteens(hand, cut);
    score += fifteens;

    let knobs = knobs(hand, cut);
    score += knobs;

    let despiration = despiration(hand, cut);
    score += despiration;

    let runs = runs(hand, cut);
    score += runs;

    score
}

fn pairs(hand: &Hand, cut: Card) -> u32 {
    let Hand(a, b, c, d) = hand;

    let buf = [a, b, c, d, &cut];

    let mut score = 0;

    for a in &buf {
        for b in &buf {
            if a.seq() == b.seq() {
                score += 2;
            }
        }
    }

    (score - 10) >> 1
}

fn fifteens(hand: &Hand, cut: Card) -> u32 {
    let Hand(a, b, c, d) = hand;

    let buf = [a.score(), b.score(), c.score(), d.score(), cut.score()].to_vec();

    let mut score = 0;

    for partition_size in 2..5 {
        for parts in buf.combs(partition_size) {
            if parts.into_iter().sum::<u32>() == 15 {
                score += 2;
            }
        }
    }

    if buf.iter().sum::<u32>() == 15 {
        score += 2;
    }

    score
}

fn knobs(hand: &Hand, cut: Card) -> u32 {
    let Hand(a, b, c, d) = hand;

    for card in [a, b, c, d].iter() {
        if card.0 == Value::Jack && card.1 == cut.1 {
            return 1;
        }
    }

    0
}

fn despiration(hand: &Hand, cut: Card) -> u32 {
    let Hand(a, b, c, d) = hand;

    let mut score = 0;

    if a.1 == b.1 && b.1 == c.1 && c.1 == d.1 {
        score += 4;
    }

    if a.1 == cut.1 {
        score += 1;
    }

    score
}

fn runs(hand: &Hand, cut: Card) -> u32 {
    let Hand(a, b, c, d) = hand;

    let mut sequence_vals = [a.0.seq(), b.0.seq(), c.0.seq(), d.0.seq(), cut.0.seq()];
    sequence_vals.sort();

    let seq = find_seqs(&sequence_vals);

    let (dups, dup_count) = dups(seq);
    let counts = seq.len() as u32;

    if dups != 0 {
        dups * (counts + dup_count - dups)
    } else {
        counts
    }
}

fn find_seqs(mut vals: &[u32]) -> &[u32] {
    while vals.len() > 2 {
        let mut value = vals[0];
        let mut idx = 0;
        for (i, val) in vals.iter().enumerate().skip(1) {
            let val = *val;
            if val != value && val != value + 1 {
                break;
            }

            if vals[i] == value + 1 {
                value += 1;
            }

            idx = i;
        }

        if idx > 1 && vals[idx] >= vals[0] + 2 {
            return &vals[0..=idx];
        }

        vals = &vals[1..];
    }

    &vals[0..0]
}

fn dups(vals: &[u32]) -> (u32, u32) {
    if vals.is_empty() {
        return (0, 0);
    }

    let mut dups = 0;
    let mut count = 0;
    let mut counts = [0; 16];

    for val in vals {
        counts[*val as usize] += 1;
    }

    for c in &counts {
        if *c > 1 {
            dups += c;
            count += 1;
        }
    }

    (dups, count)
}

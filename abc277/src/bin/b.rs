use std::fmt;

use itertools::Itertools;
use proconio::{input, marker::Chars};

const SUITS: [char; 4] = ['H', 'D', 'C', 'S'];

const RANKS: [char; 13] = [
    'A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K',
];

fn main() {
    input! {
        n: u8,
        ss: [Chars; n],
    }

    println!("{}", solve(&ss));
}

fn solve(ss: &[Vec<char>]) -> impl fmt::Display {
    if is_playing_cards(ss) {
        "Yes"
    } else {
        "No"
    }
}

fn is_playing_cards(ss: &[Vec<char>]) -> bool {
    ss.iter()
        .all(|s| SUITS.contains(&s[0]) && RANKS.contains(&s[1]))
        && ss.iter().tuple_combinations().all(|(s1, s2)| s1 != s2)
}

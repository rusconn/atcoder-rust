use std::fmt;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    println!("{}", solve(&s));
}

fn solve(s: &[char]) -> impl fmt::Display {
    s.iter()
        .rposition(|&c| c == 'a')
        .map_or(-1, |p| p as i8 + 1)
}

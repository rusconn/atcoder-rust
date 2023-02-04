use std::fmt;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    println!("{}", solve(&s));
}

fn solve(s: &[char]) -> impl fmt::Display {
    s[s.len() / 2]
}

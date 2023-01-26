use std::fmt;

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!("{}", solve(&s));
}

fn solve(s: &str) -> impl fmt::Display {
    s.chars().map(points).sum::<u8>()
}

fn points(c: char) -> u8 {
    if c == 'v' {
        1
    } else {
        2
    }
}

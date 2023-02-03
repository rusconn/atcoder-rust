use std::fmt;

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!("{}", solve(&s));
}

fn solve(s: &str) -> impl fmt::Display {
    5 - ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"]
        .iter()
        .position(|&day| day == s)
        .unwrap()
}

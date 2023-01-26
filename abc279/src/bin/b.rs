use std::fmt;

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    println!("{}", solve(&s, &t));
}

fn solve(s: &str, t: &str) -> impl fmt::Display {
    if s.contains(t) {
        "Yes"
    } else {
        "No"
    }
}

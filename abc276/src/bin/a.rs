use std::fmt;

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!("{}", solve(&s));
}

fn solve(s: &str) -> impl fmt::Display {
    s.rfind('a').map_or(-1, |i| i as i8 + 1)
}

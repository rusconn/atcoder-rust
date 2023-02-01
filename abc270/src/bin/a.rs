use std::fmt;

use proconio::input;

fn main() {
    input! {
        a: u8,
        b: u8,
    }

    println!("{}", solve(a, b));
}

fn solve(a: u8, b: u8) -> impl fmt::Display {
    a | b
}

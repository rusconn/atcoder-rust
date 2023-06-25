use std::{fmt, i8};

use proconio::input;

fn main() {
    input! {
        r: i8,
        c: i8,
    }

    println!("{}", solve(r, c));
}

fn solve(r: i8, c: i8) -> impl fmt::Display {
    let d = i8::max((8 - r).abs(), (8 - c).abs());

    if d % 2 == 0 {
        "white"
    } else {
        "black"
    }
}

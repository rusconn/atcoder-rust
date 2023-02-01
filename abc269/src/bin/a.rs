use std::fmt;

use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }

    println!("{}", solve(a, b, c, d));
}

fn solve(a: i32, b: i32, c: i32, d: i32) -> impl fmt::Display {
    format!("{}\nTakahashi", (a + b) * (c - d))
}

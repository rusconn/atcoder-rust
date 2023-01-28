use std::fmt;

use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    println!("{:.3}", solve(a, b));
}

fn solve(a: f64, b: f64) -> impl fmt::Display {
    b / a
}

use std::fmt;

use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    println!("{}", solve(n));
}

fn solve(n: i32) -> impl fmt::Display {
    n
}

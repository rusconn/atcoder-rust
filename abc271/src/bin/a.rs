use std::fmt;

use proconio::input;

fn main() {
    input! {
        n: u8,
    }

    println!("{}", solve(n));
}

fn solve(n: u8) -> impl fmt::Display {
    format!("{:02X}", n)
}

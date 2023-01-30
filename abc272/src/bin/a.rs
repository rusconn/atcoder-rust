use std::fmt;

use proconio::input;

fn main() {
    input! {
        n: u8,
        a: [u16; n],
    }

    println!("{}", solve(&a));
}

fn solve(a: &[u16]) -> impl fmt::Display {
    a.iter().sum::<u16>()
}

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u8,
    }

    println!("{}", solve(n));
}

fn solve(n: u8) -> impl std::fmt::Display {
    (0..=n).rev().join("\n")
}

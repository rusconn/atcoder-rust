use std::{fmt, iter};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u8,
        k: usize,
        a: [u8; n],
    }

    println!("{}", solve(k, &a));
}

fn solve(k: usize, a: &[u8]) -> impl fmt::Display {
    a.iter()
        .chain(iter::repeat(&0u8))
        .skip(k)
        .take(a.len())
        .join(" ")
}

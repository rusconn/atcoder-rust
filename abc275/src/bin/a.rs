use std::fmt;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u8,
        hs: [u32; n],
    }

    println!("{}", solve(&hs));
}

fn solve(hs: &[u32]) -> impl fmt::Display {
    hs.iter().position_max().unwrap() + 1
}

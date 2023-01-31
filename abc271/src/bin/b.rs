use std::fmt;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: u32,
        q: u32,
        ass: [[u32]; n],
        sts: [(Usize1, Usize1); q],
    }

    println!("{}", solve(&ass, &sts));
}

fn solve(ass: &[Vec<u32>], sts: &[(usize, usize)]) -> impl fmt::Display {
    sts.iter().map(|&(s, t)| ass[s][t]).join("\n")
}

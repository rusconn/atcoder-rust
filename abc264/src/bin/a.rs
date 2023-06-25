use std::fmt;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        l: Usize1,
        r: Usize1,
    }

    println!("{}", solve(l, r));
}

fn solve(l: usize, r: usize) -> impl fmt::Display {
    &"atcoder"[l..=r]
}

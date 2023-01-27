use std::fmt;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u8,
        x: u8,
        ps: [u8; n],
    }

    println!("{}", solve(&ps, x));
}

fn solve(ps: &[u8], x: u8) -> impl fmt::Display {
    ps.iter().find_position(|&&p| p == x).unwrap().0 + 1
}

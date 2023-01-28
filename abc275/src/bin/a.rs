use std::fmt;

use proconio::input;

fn main() {
    input! {
        n: u8,
        hs: [u32; n],
    }

    println!("{}", solve(&hs));
}

fn solve(hs: &[u32]) -> impl fmt::Display {
    hs.iter().enumerate().max_by_key(|p| p.1).unwrap().0 + 1
}

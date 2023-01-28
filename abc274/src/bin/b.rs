use std::fmt;

use itertools::Itertools;
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        h: usize,
        w: usize,
        css: [Bytes; h],
    }

    println!("{}", solve(w, &css));
}

fn solve(w: usize, css: &[Vec<u8>]) -> impl fmt::Display {
    (0..w)
        .map(|j| css.iter().filter(|cs| cs[j] == b'#').count())
        .join(" ")
}

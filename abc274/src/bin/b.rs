use std::fmt;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        css: [Chars; h],
    }

    println!("{}", solve(w, &css));
}

fn solve(w: usize, css: &[Vec<char>]) -> impl fmt::Display {
    (0..w)
        .map(|j| css.iter().filter(|cs| cs[j] == '#').count())
        .join(" ")
}

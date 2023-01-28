use std::fmt;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: u16,
        _: u16,
        css: [Chars; h],
    }

    println!("{}", solve(&css));
}

fn solve(css: &[Vec<char>]) -> impl fmt::Display {
    transpose(css)
        .iter()
        .map(|v| v.iter().filter(|&&c| c == '#').count())
        .join(" ")
}

fn transpose<T: Copy + Default>(vs: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut vs_t = vec![vec![T::default(); vs.len()]; vs[0].len()];

    for (i, v) in vs.iter().enumerate() {
        for j in 0..v.len() {
            vs_t[j][i] = v[j];
        }
    }

    vs_t
}

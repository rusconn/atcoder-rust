use std::fmt;

use itertools::Itertools;
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    println!("{}", solve(n, &a));
}

fn solve(n: usize, a: &[u32]) -> impl fmt::Display {
    let mut histogram = FxHashMap::default();

    for &x in a {
        *histogram.entry(x).or_insert(0) += 1;
    }

    let greaters = a.iter().sorted().rev().dedup().collect_vec();

    (0..n)
        .map(|k| greaters.get(k).map_or(0, |v| *histogram.get(v).unwrap()))
        .join("\n")
}

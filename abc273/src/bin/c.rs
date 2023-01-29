use std::{collections::BTreeMap, fmt, iter};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    println!("{}", solve(&a));
}

fn solve(a: &[u32]) -> impl fmt::Display {
    let mut histogram = BTreeMap::default();

    for &x in a {
        *histogram.entry(x).or_insert(0) += 1;
    }

    histogram
        .iter()
        .rev()
        .map(|(_, &v)| v)
        .chain(iter::repeat(0))
        .take(a.len())
        .join("\n")
}

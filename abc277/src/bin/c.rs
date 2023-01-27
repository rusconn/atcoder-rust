use std::fmt;

use proconio::input;
use rustc_hash::{FxHashMap, FxHashSet};

fn main() {
    input! {
        n: u32,
        abs: [(u32, u32); n],
    }

    println!("{}", solve(&abs));
}

fn solve(abs: &[(u32, u32)]) -> impl fmt::Display {
    let mut ladders = FxHashMap::default();
    let mut visiteds = FxHashSet::default();
    let mut remainings = vec![1];

    for &(a, b) in abs {
        ladders.entry(a).or_insert(vec![]).push(b);
        ladders.entry(b).or_insert(vec![]).push(a);
    }

    while let Some(x) = remainings.pop() {
        visiteds.insert(x);

        if let Some(ys) = ladders.get(&x) {
            ys.iter()
                .filter(|y| !visiteds.contains(y))
                .for_each(|&y| remainings.push(y));
        }
    }

    visiteds.into_iter().max().unwrap()
}

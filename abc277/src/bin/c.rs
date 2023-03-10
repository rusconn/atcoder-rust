use std::fmt;

use proconio::input;
use rustc_hash::{FxHashMap, FxHashSet};

fn main() {
    input! {
        n: u32,
    }

    let mut ladders = FxHashMap::default();

    for _ in 0..n {
        input! {
            a: u32,
            b: u32,
        }

        ladders.entry(a).or_insert(vec![]).push(b);
        ladders.entry(b).or_insert(vec![]).push(a);
    }

    println!("{}", solve(&ladders));
}

fn solve(ladders: &FxHashMap<u32, Vec<u32>>) -> impl fmt::Display {
    let mut visiteds = FxHashSet::default();
    let mut remainings = vec![1];

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

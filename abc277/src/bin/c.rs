use std::fmt;

use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        n: u32,
        abs: [(u32, u32); n],
    }

    println!("{}", solve(&abs));
}

fn solve(abs: &[(u32, u32)]) -> impl fmt::Display {
    let mut top = 0;
    let mut visited = FxHashSet::default();
    let mut remaining = vec![1];

    while let Some(x) = remaining.pop() {
        top = top.max(x);
        visited.insert(x);

        abs.iter()
            .filter(|&(a, b)| *a == x && !visited.contains(b))
            .for_each(|&(_, b)| remaining.push(b));
    }

    top
}

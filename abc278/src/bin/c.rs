use std::fmt;

use proconio::input;
use rustc_hash::{FxHashMap, FxHashSet};

fn main() {
    input! {
        _: u32,
        q: u32,
        tabs: [(u8, usize, usize); q],
    }

    println!("{}", solve(&tabs));
}

fn solve(tabs: &[(u8, usize, usize)]) -> impl fmt::Display {
    let mut follows: FxHashMap<usize, FxHashSet<usize>> = FxHashMap::default();
    let mut answers: Vec<&str> = vec![];

    for &(t, a, b) in tabs {
        match t {
            1 => {
                follows
                    .entry(a)
                    .or_insert_with(FxHashSet::default)
                    .insert(b);
            }
            2 => {
                follows
                    .entry(b)
                    .or_insert_with(FxHashSet::default)
                    .remove(&a);
            }
            _ => {
                if follows.get(&a).map_or(false, |s| s.contains(&b))
                    && follows.get(&b).map_or(false, |s| s.contains(&a))
                {
                    answers.push("Yes");
                } else {
                    answers.push("No");
                }
            }
        }
    }

    answers.join("\n")
}

use std::fmt;

use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        _: u32,
        q: u32,
        tabs: [(u8, u64, u64); q],
    }

    println!("{}", solve(&tabs));
}

fn solve(tabs: &[(u8, u64, u64)]) -> impl fmt::Display {
    let mut follows = FxHashSet::default();
    let mut answers = vec![];

    for &(t, a, b) in tabs {
        match t {
            1 => {
                follows.insert((a, b));
            }
            2 => {
                follows.remove(&(a, b));
            }
            _ => {
                if follows.contains(&(a, b)) && follows.contains(&(b, a)) {
                    answers.push("Yes");
                } else {
                    answers.push("No");
                }
            }
        }
    }

    answers.join("\n")
}

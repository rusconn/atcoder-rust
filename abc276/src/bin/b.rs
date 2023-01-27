use std::{collections::BTreeSet, fmt};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u32,
    }

    let mut roads_map = vec![BTreeSet::default(); n];

    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }

        roads_map[a - 1].insert(b);
        roads_map[b - 1].insert(a);
    }

    println!("{}", solve(&roads_map));
}

fn solve(roads_map: &[BTreeSet<usize>]) -> impl fmt::Display {
    roads_map
        .iter()
        .map(|roads| {
            if roads.is_empty() {
                "0".to_string()
            } else {
                format!("{} {}", roads.len(), roads.iter().join(" "))
            }
        })
        .join("\n")
}

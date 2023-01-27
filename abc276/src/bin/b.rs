use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
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

    for roads in roads_map {
        if roads.is_empty() {
            println!(0);
        } else {
            println!("{} {}", roads.len(), roads.iter().join(" "));
        }
    }
}

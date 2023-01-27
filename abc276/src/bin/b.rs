use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{fastout, input};
use rustc_hash::FxHashMap;

#[fastout]
fn main() {
    input! {
        n: u32,
        m: u32,
    }

    let mut roads_map = FxHashMap::default();

    for _ in 0..m {
        input! {
            a: u32,
            b: u32,
        }

        roads_map
            .entry(a)
            .or_insert_with(BTreeSet::default)
            .insert(b);

        roads_map
            .entry(b)
            .or_insert_with(BTreeSet::default)
            .insert(a);
    }

    for i in 1..=n {
        if let Some(roads) = roads_map.get(&i) {
            println!("{} {}", roads.len(), roads.iter().join(" "));
        } else {
            println!(0);
        }
    }
}

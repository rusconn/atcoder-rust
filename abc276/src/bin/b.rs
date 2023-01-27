use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: u32,
    }

    let mut roads_map: Vec<BTreeSet<_>> = Vec::with_capacity(n + 1);

    unsafe {
        roads_map.set_len(n + 1);
    }

    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }

        if let Some(set) = roads_map.get_mut(a) {
            set.insert(b);
        } else {
            roads_map[a] = BTreeSet::default();
        }

        if let Some(set) = roads_map.get_mut(b) {
            set.insert(a);
        } else {
            roads_map[b] = BTreeSet::default();
        }
    }

    for i in 1..=n {
        if let Some(roads) = roads_map.get(i) {
            println!("{} {}", roads.len(), roads.iter().join(" "));
        } else {
            println!(0);
        }
    }
}

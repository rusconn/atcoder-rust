use std::fmt;

use itertools::Itertools;
use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        uvs: [(usize, usize); n - 1],
    }

    println!("{}", solve(n, x, y, &uvs));
}

fn solve(n: usize, x: usize, y: usize, uvs: &[(usize, usize)]) -> impl fmt::Display {
    let mut stack = vec![x];
    let mut edges = vec![FxHashSet::default(); n + 1];

    for &(u, v) in uvs {
        edges[u].insert(v);
        edges[v].insert(u);
    }

    while !stack.is_empty() {
        let &arrival = stack.last().unwrap();

        if arrival == y {
            break;
        }

        let next_edges = &mut edges[arrival];

        if let Some(&next) = next_edges.iter().next() {
            next_edges.remove(&next);
            edges[next].remove(&arrival);
            stack.push(next);
        } else {
            stack.pop();
        }
    }

    stack.iter().join(" ")
}

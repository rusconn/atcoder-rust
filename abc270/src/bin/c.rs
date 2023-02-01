use std::fmt;

use itertools::Itertools;
use proconio::input;

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
    let mut edges = vec![vec![]; n + 1];

    for &(u, v) in uvs {
        edges[u].push(v);
        edges[v].push(u);
    }

    let mut stack = vec![vec![x]];

    while let Some(path) = stack.pop() {
        let &arrival = path.last().unwrap();

        if arrival == y {
            return path.iter().join(" ");
        }

        for &edge in &edges[arrival] {
            if !path.contains(&edge) {
                stack.push(path.iter().chain(&[edge]).copied().collect());
            }
        }
    }

    unreachable!()
}

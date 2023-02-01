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
    let mut stack = vec![x];
    let mut visited = vec![false; n + 1];
    let mut edges = vec![vec![]; n + 1];

    for &(u, v) in uvs {
        edges[u].push(v);
        edges[v].push(u);
    }

    while !stack.is_empty() {
        let &arrival = stack.last().unwrap();
        visited[arrival] = true;

        if arrival == y {
            break;
        } else if let Some(&x) = edges[arrival].iter().find(|&&v| !visited[v]) {
            stack.push(x);
        } else {
            stack.pop();
        }
    }

    stack.iter().join(" ")
}

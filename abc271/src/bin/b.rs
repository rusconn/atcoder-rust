use std::fmt;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut ass = Vec::with_capacity(n);

    for _ in 0..n {
        input! {
            l: u32,
            a: [u32; l],
        }

        ass.push(a);
    }

    let mut sts = Vec::with_capacity(q);

    for _ in 0..q {
        input! {
            s: Usize1,
            t: Usize1,
        }

        sts.push((s, t));
    }

    println!("{}", solve(&ass, &sts));
}

fn solve(ass: &[Vec<u32>], sts: &[(usize, usize)]) -> impl fmt::Display {
    sts.iter().map(|&(s, t)| ass[s][t]).join("\n")
}

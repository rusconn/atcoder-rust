use std::fmt;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u32; n],
    }

    println!("{}", solve(&a));
}

fn solve(a: &[u32]) -> impl fmt::Display {
    let mut map = vec![0u32; 2 * a.len() + 1];

    for (i, &x) in a.iter().enumerate() {
        let h = map[x as usize - 1] + 1;
        let i = 2 * i;
        map[i + 1] = h;
        map[i + 2] = h;
    }

    map.iter().join("\n")
}

use std::fmt;

use proconio::{input, marker::Bytes};

fn main() {
    input! {
        ss: [Bytes; 10],
    }

    println!("{}", solve(&ss));
}

fn solve(ss: &[Vec<u8>]) -> impl fmt::Display {
    let a = ss.iter().position(|s| s.contains(&b'#')).unwrap() + 1;
    let b = ss.iter().rposition(|s| s.contains(&b'#')).unwrap() + 1;
    let c = ss[a - 1].iter().position(|&c| c == b'#').unwrap() + 1;
    let d = ss[a - 1].iter().rposition(|&c| c == b'#').unwrap() + 1;

    format!("{} {}\n{} {}", a, b, c, d)
}

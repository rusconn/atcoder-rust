use std::fmt;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: u32,
        _: u32,
        ss: [Chars; h],
        ts: [Chars; h],
    }

    println!("{}", solve(&ss, &ts));
}

fn solve(ss: &[Vec<char>], ts: &[Vec<char>]) -> impl fmt::Display {
    let mut ss_t = transpose(&ss);
    let mut ts_t = transpose(&ts);

    ss_t.sort_unstable();
    ts_t.sort_unstable();

    if ss_t == ts_t {
        "Yes"
    } else {
        "No"
    }
}

fn transpose<T: Copy + Default>(vs: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut vs_t = vec![vec![T::default(); vs.len()]; vs[0].len()];

    for (i, v) in vs.iter().enumerate() {
        for j in 0..v.len() {
            vs_t[j][i] = v[j];
        }
    }

    vs_t
}

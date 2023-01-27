use std::fmt;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u8,
        mut ps: [u8; n],
    }

    println!("{}", solve(&mut ps));
}

fn solve(ps: &mut [u8]) -> impl fmt::Display {
    let mut i = 0;

    for j in 1..ps.len() {
        if is_sorted(&ps[j..]) {
            break;
        }
        i = j;
    }

    let j = pred_index(&ps, i);

    ps.swap(i, j);

    let (l, r) = ps.split_at_mut(i + 1);

    r.reverse();

    l.iter_mut().chain(r).join(" ")
}

fn is_sorted(xs: &[u8]) -> bool {
    if xs.is_empty() {
        return true;
    }

    let mut pred = xs[0];

    for &x in xs.iter().skip(1) {
        if pred > x {
            return false;
        }
        pred = x;
    }

    true
}

fn pred_index(xs: &[u8], i: usize) -> usize {
    let mut pred = 0;
    let mut pred_i = 0;

    for j in i + 1..xs.len() {
        if xs[i] > xs[j] && pred < xs[j] {
            pred = xs[j];
            pred_i = j;
        }
    }

    pred_i
}

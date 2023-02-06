use std::{fmt, iter::FromIterator};

use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        t: usize,
        a: [usize; n - 1],
        xys: [(usize, usize); m],
    }

    let xys = FxHashMap::from_iter(xys);

    println!("{}", solve(t, &a, &xys));
}

fn solve(t: usize, a: &[usize], xys: &FxHashMap<usize, usize>) -> impl fmt::Display {
    if is_reachable(t, a, xys) {
        "Yes"
    } else {
        "No"
    }
}

fn is_reachable(mut t: usize, a: &[usize], xys: &FxHashMap<usize, usize>) -> bool {
    for (i, &ai) in a.iter().enumerate() {
        if t <= ai {
            return false;
        }

        t -= ai;
        t += xys.get(&(i + 2)).unwrap_or(&0);
    }

    true
}

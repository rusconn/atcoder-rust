use std::{cmp::Reverse, fmt};

use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [i32; n],
    }

    println!("{}", solve(&a));
}

fn solve(a: &[i32]) -> impl fmt::Display {
    if let Some(x) = max_even(a) {
        x
    } else {
        -1
    }
}

fn max_even(a: &[i32]) -> Option<i32> {
    let (mut evens, mut odds): (Vec<i32>, Vec<i32>) = a.iter().partition(|&&ai| ai % 2 == 0);

    evens.sort_unstable_by_key(|&x| Reverse(x));
    odds.sort_unstable_by_key(|&x| Reverse(x));

    let mut max_even = None;

    if let (Some(x), Some(y)) = (evens.get(0), evens.get(1)) {
        max_even = Some(x + y);
    }

    if let (Some(x), Some(y)) = (odds.get(0), odds.get(1)) {
        max_even = max_even.max(Some(x + y));
    }

    max_even
}

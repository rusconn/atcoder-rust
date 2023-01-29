use std::fmt;

use num_integer::div_rem;
use proconio::input;

fn main() {
    input! {
        x: u64,
        k: u32,
    }

    println!("{}", solve(x, k));
}

fn solve(mut x: u64, k: u32) -> impl fmt::Display {
    for _ in 0..k {
        let (d, r) = div_rem(x, 10);
        x = d + if r < 5 { 0 } else { 1 };
    }

    x * 10u64.pow(k)
}

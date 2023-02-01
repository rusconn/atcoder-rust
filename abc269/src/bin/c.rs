use std::fmt;

use itertools::Itertools;
use num_integer::div_rem;
use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    println!("{}", solve(n));
}

fn solve(mut n: u64) -> impl fmt::Display {
    let mut bits = vec![];

    while n != 0 {
        let (d, r) = div_rem(n, 2);
        n = d;
        bits.push(r);
    }

    let mut nums = vec![0u64];

    for (i, &b) in bits.iter().enumerate() {
        if b == 1 {
            nums.append(&mut nums.iter().map(|n| n + 2u64.pow(i as u32)).collect());
        }
    }

    nums.iter().join("\n")
}

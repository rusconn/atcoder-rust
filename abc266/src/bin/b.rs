use std::fmt;

use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    println!("{}", solve(n));
}

fn solve(n: i64) -> impl fmt::Display {
    let r = n % 998244353;

    if r < 0 {
        998244353 + r
    } else {
        r
    }
}

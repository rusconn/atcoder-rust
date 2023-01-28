use std::fmt;

use proconio::input;

fn main() {
    input! {
        abcdef: [u128; 6]
    }

    println!("{}", solve(&abcdef));
}

fn solve(abcdef: &[u128]) -> impl fmt::Display {
    let rs: Vec<_> = abcdef.iter().map(|x| x % 998244353).collect();
    (rs[0] * rs[1] * rs[2] - rs[3] * rs[4] * rs[5]) % 998244353
}

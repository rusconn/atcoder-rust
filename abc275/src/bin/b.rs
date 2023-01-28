use std::fmt;

use proconio::input;

const DIVISOR: u128 = 998244353;

fn main() {
    input! {
        abcdef: [u128; 6]
    }

    println!("{}", solve(&abcdef));
}

fn solve(abcdef: &[u128]) -> impl fmt::Display {
    let rs: Vec<_> = abcdef.iter().map(|x| x % DIVISOR).collect();
    let abc = rs[0] * rs[1] * rs[2] % DIVISOR;
    let def = rs[3] * rs[4] * rs[5] % DIVISOR;
    (abc + DIVISOR - def) % DIVISOR
}

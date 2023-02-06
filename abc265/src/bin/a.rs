use std::fmt;

use num::Integer;
use proconio::input;

fn main() {
    input! {
        x: u16,
        y: u16,
        n: u16,
    }

    println!("{}", solve(x, y, n));
}

fn solve(x: u16, y: u16, n: u16) -> impl fmt::Display {
    let (yc, xc) = n.div_rem(&3);
    (x * n).min(y * yc + x * xc)
}

use std::fmt;

use permutohedron::factorial;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", solve(n));
}

fn solve(n: usize) -> impl fmt::Display {
    factorial(n)
}

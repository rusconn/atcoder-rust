use std::fmt;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    println!("{}", solve(n, &a));
}

fn solve(n: usize, a: &[usize]) -> impl fmt::Display {
    let mut books = vec![false; n];

    for &ai in a {
        if ai < n {
            books[ai] = true;
        }
    }

    let mut n = n as i32;
    let mut read = 0;

    while n >= 0 {
        n -= if books[read] { 1 } else { 2 };
        read += 1;
    }

    read - 1
}

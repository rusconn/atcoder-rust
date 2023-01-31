use std::{fmt, iter::FromIterator};

use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        n: u32,
        a: [u32; n],
    }

    println!("{}", solve(&a));
}

fn solve(a: &[u32]) -> impl fmt::Display {
    let books = FxHashSet::from_iter(a);
    let mut rest = a.len() as i32;
    let mut read = 0;

    while rest >= 0 {
        read += 1;
        rest -= if books.contains(&read) { 1 } else { 2 };
    }

    read - 1
}

use std::fmt;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        css: [Chars; h],
    }

    println!("{}", solve(w, &css));
}

fn solve(w: usize, css: &[Vec<char>]) -> impl fmt::Display {
    let mut counts = vec![0; w];

    for cs in css.iter() {
        for j in 0..w {
            if cs[j] == '#' {
                counts[j] += 1;
            }
        }
    }

    counts.iter().join(" ")
}

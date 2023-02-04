use std::fmt;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    }

    println!("{}", solve(m, &a));
}

fn solve(m: usize, a: &[i64]) -> impl fmt::Display {
    let mut sum1: i64 = a.iter().take(m).sum();

    let mut sum: i64 = a
        .iter()
        .take(m)
        .enumerate()
        .map(|(i, bi)| (i + 1) as i64 * bi)
        .sum();

    let mut max_sum = sum;

    for i in 0..(a.len() - m) {
        sum = sum - sum1 + (m as i64) * a[m + i];
        max_sum = max_sum.max(sum);
        sum1 = sum1 - a[i] + a[m + i];
    }

    max_sum
}

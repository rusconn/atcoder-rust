use std::{fmt, i64};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    }

    println!("{}", solve(n, m, &a));
}

fn solve(n: usize, m: usize, a: &[i64]) -> impl fmt::Display {
    let mut dp = vec![vec![i64::MIN; n + 1]; m + 1];

    for j in 0..=n {
        dp[0][j] = 0;
    }

    for i in 1..=m {
        for j in i..=n {
            dp[i][j] = dp[i][j - 1].max(dp[i - 1][j - 1] + i as i64 * a[j - 1]);
        }
    }

    dp.into_iter().flatten().max().unwrap()
}

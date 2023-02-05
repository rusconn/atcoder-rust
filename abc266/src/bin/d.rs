use std::fmt;

use proconio::input;

fn main() {
    input! {
        n: usize,
        txas: [(usize, usize, usize); n],
    }

    println!("{}", solve(&txas));
}

fn solve(txas: &[(usize, usize, usize)]) -> impl fmt::Display {
    let max_t = txas.last().unwrap().0;
    let mut dp = vec![vec![0; 5 + 2]; max_t + 1];

    for &(t, x, a) in txas {
        dp[t][x + 1] = a;
    }

    for i in 0..(max_t + 1).min(5) {
        for j in 2 + i..7 {
            dp[i][j] = 0;
        }
    }

    for i in 1..=max_t {
        for j in 1..=5 {
            dp[i][j] += *[dp[i - 1][j - 1], dp[i - 1][j], dp[i - 1][j + 1]]
                .iter()
                .max()
                .unwrap();
        }
    }

    *dp[max_t].iter().max().unwrap()
}

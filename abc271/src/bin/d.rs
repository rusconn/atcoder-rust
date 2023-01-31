use std::fmt;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        abs: [(usize, usize); n],
    }

    println!("{}", solve(n, s, &abs));
}

fn solve(n: usize, s: usize, abs: &[(usize, usize)]) -> impl fmt::Display {
    if let Some(th) = is_adjustable(n, s, abs) {
        format!("Yes\n{}", th)
    } else {
        "No".to_string()
    }
}

fn is_adjustable(n: usize, s: usize, abs: &[(usize, usize)]) -> Option<String> {
    let mut dp = vec![vec![false; s + 1]; n + 1];

    dp[1][abs[0].0] = true;
    dp[1][abs[0].1] = true;

    for (i, &(a, b)) in abs.iter().enumerate().skip(1) {
        for j in 1..=s {
            if dp[i][j] {
                if j + a <= s {
                    dp[i + 1][j + a] = true;
                }
                if j + b <= s {
                    dp[i + 1][j + b] = true;
                }
            }
        }
    }

    if !dp[n][s] {
        return None;
    }

    let mut ths = vec![];
    let mut j = s;

    for (i, &(a, b)) in abs.iter().enumerate().skip(1).rev() {
        if dp[i][j - a] {
            ths.push('H');
            j -= a;
        } else {
            ths.push('T');
            j -= b;
        }
    }

    ths.push(if j == abs[0].0 { 'H' } else { 'T' });

    Some(ths.iter().rev().join(""))
}

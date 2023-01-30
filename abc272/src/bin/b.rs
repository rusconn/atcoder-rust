use std::fmt;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut xss = vec![];

    for _ in 0..m {
        input! {
            k: u8,
            xs: [Usize1; k],
        }

        xss.push(xs);
    }

    println!("{}", solve(n, &xss));
}

fn solve(n: usize, xss: &[Vec<usize>]) -> impl fmt::Display {
    if is_all_friends(n, xss) {
        "Yes"
    } else {
        "No"
    }
}

fn is_all_friends(n: usize, xss: &[Vec<usize>]) -> bool {
    let mut vs = vec![vec![false; n]; n];

    for xs in xss {
        for &x in xs {
            for &y in xs {
                vs[x][y] = true;
            }
        }
    }

    vs.iter().all(|v| v.iter().all(|&b| b))
}

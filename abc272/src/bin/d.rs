use std::{collections::VecDeque, fmt};

use itertools::Itertools;
use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    println!("{}", solve(n, m));
}

fn solve(n: usize, m: usize) -> impl fmt::Display {
    let mut counts = vec![vec![-1; n]; n];
    let mut arrivals = VecDeque::default();

    counts[0][0] = 0i16;
    arrivals.push_back((0usize, 0usize));

    while let Some((i, j)) = arrivals.pop_front() {
        let next_count = counts[i][j] + 1;
        let mut moves = vec![];

        for y in 0..=m.sqrt() {
            let rest = m - y.pow(2);
            if rest.is_power_of_two() {
                let x = rest.sqrt();
                moves.push((y, x));
                moves.push((x, y));
            }
        }

        for (y, x) in moves {
            let mut next_coords = vec![];

            if y <= i && x <= j {
                next_coords.push((i - y, j - x));
            }
            if y <= i && j + x < n {
                next_coords.push((i - y, j + x));
            }
            if i + y < n && x <= j {
                next_coords.push((i + y, j - x));
            }
            if i + y < n && j + x < n {
                next_coords.push((i + y, j + x));
            }

            for (a, b) in next_coords {
                let next_cell = &mut counts[a][b];

                if *next_cell == -1 || *next_cell > next_count {
                    arrivals.push_back((a, b));
                    *next_cell = next_count;
                }
            }
        }
    }

    counts.iter().map(|v| v.iter().join(" ")).join("\n")
}

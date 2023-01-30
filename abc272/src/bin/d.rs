use std::{collections::VecDeque, fmt};

use itertools::Itertools;
use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i32,
    }

    println!("{}", solve(n, m));
}

fn solve(n: usize, m: i32) -> impl fmt::Display {
    let mut counts = vec![vec![-1; n]; n];
    let mut arrivals = VecDeque::default();
    let mut dyxs = vec![];

    counts[0][0] = 0i16;
    arrivals.push_back((0i32, 0i32));

    for dy in 0..=m.sqrt() {
        let rest = m - dy.pow(2);
        let dx = rest.sqrt();
        if dy.pow(2) + dx.pow(2) == m {
            dyxs.push((-dy, -dx));
            dyxs.push((-dy, dx));
            dyxs.push((dy, -dx));
            dyxs.push((dy, dx));
        }
    }

    while let Some((y, x)) = arrivals.pop_front() {
        let next_count = counts[y as usize][x as usize] + 1;

        for &(dy, dx) in &dyxs {
            let y = y + dy;
            let x = x + dx;

            if y < 0 || y >= n as i32 || x < 0 || x >= n as i32 {
                continue;
            }

            let next_cell = &mut counts[y as usize][x as usize];

            if *next_cell == -1 || *next_cell > next_count {
                arrivals.push_back((y, x));
                *next_cell = next_count;
            }
        }
    }

    counts.iter().map(|v| v.iter().join(" ")).join("\n")
}

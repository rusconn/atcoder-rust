use std::fmt;

use itertools::Itertools;
use proconio::input;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    White,
    Black,
    Count,
}

fn main() {
    input! {
        n: u16,
        xys: [(i16, i16); n],
    }

    println!("{}", solve(&xys));
}

fn solve(xys: &[(i16, i16)]) -> impl fmt::Display {
    let xys = xys
        .iter()
        .map(|(x, y)| ((x + 1000) as usize, (y + 1000) as usize))
        .collect_vec();

    let mut map = vec![vec![Cell::White; 2000 + 1]; 2000 + 1];

    for &(x, y) in &xys {
        map[x][y] = Cell::Black;
    }

    let mut count = 0;

    for &(x, y) in &xys {
        if map[x][y] == Cell::Black {
            let mut stack = vec![(x as i16, y as i16)];

            while let Some((x, y)) = stack.pop() {
                map[x as usize][y as usize] = Cell::Count;

                for &(x, y) in [
                    (x - 1, y - 1),
                    (x - 1, y),
                    (x, y - 1),
                    (x, y + 1),
                    (x + 1, y),
                    (x + 1, y + 1),
                ]
                .iter()
                .filter(|&&(x, y)| 0 <= x && x <= 2000 && 0 <= y && y <= 2000)
                .filter(|&&(x, y)| map[x as usize][y as usize] == Cell::Black)
                {
                    stack.push((x, y));
                }
            }

            count += 1;
        }
    }

    count
}

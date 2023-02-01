use std::fmt;

use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        n: u16,
        xys: [(i16, i16); n],
    }

    println!("{}", solve(&xys));
}

fn solve(xys: &[(i16, i16)]) -> impl fmt::Display {
    let mut seen = FxHashSet::default();
    let mut count = 0;

    for &xy in xys {
        if !seen.insert(xy) {
            continue;
        }

        let mut stack = vec![xy];

        while let Some((x, y)) = stack.pop() {
            for &xy in &[
                (x - 1, y - 1),
                (x - 1, y),
                (x, y - 1),
                (x, y + 1),
                (x + 1, y),
                (x + 1, y + 1),
            ] {
                if xys.contains(&xy) && seen.insert(xy) {
                    stack.push(xy);
                }
            }
        }

        count += 1;
    }

    count
}

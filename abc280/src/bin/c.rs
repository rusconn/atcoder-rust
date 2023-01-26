use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    println!("{}", solve(&s, &t));
}

fn solve(s: &str, t: &str) -> impl std::fmt::Display {
    s.chars()
        .zip(t.chars())
        .chain(std::iter::once(('x', 'y')))
        .find_position(|(sc, tc)| sc != tc)
        .unwrap()
        .0
        + 1
}

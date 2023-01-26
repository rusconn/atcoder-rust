use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u8,
        ss: [i32; n],
    }

    println!("{}", solve(&ss));
}

fn solve(ss: &[i32]) -> impl std::fmt::Display {
    std::iter::once(&0)
        .chain(ss.iter())
        .tuple_windows()
        .map(|(si, sj)| sj - si)
        .join(" ")
}

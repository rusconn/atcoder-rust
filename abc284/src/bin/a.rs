use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u32,
        ss: [String; n],
    }

    println!("{}", solve(&ss));
}

fn solve(ss: &[String]) -> impl std::fmt::Display {
    ss.iter().rev().join("\n")
}

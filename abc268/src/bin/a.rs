use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        abcde: [u8; 5],
    }

    println!("{}", solve(&abcde));
}

fn solve(abcde: &[u8]) -> impl std::fmt::Display {
    abcde.iter().unique().count()
}

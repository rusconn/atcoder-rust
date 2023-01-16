use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        k: u8,
    }

    println!("{}", solve(k));
}

fn solve(k: u8) -> impl std::fmt::Display {
    (0..k).map(|n| char::from(b'A' + n)).join("")
}

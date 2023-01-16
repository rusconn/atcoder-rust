use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        _n: u32,
        s: String,
    }

    println!("{}", solve(&s));
}

fn solve(s: &str) -> impl std::fmt::Display {
    s.split('"')
        .enumerate()
        .map(|(i, x)| {
            if i % 2 == 0 {
                x.replace(',', ".")
            } else {
                x.to_string()
            }
        })
        .join("\"")
}

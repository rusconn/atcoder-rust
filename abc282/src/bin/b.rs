use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u8,
        _m: u8,
        ss: [String; n],
    }

    println!("{}", solve(&ss));
}

fn solve(ss: &[String]) -> impl std::fmt::Display {
    ss.iter()
        .combinations(2)
        .map(|v| (v[0], v[1]))
        .filter(|(x, y)| {
            x.chars()
                .zip(y.chars())
                .all(|(xc, yc)| xc == 'o' || yc == 'o')
        })
        .count()
}

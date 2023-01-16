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
        .filter(|v| {
            let x = v[0];
            let y = v[1];

            x.chars()
                .zip(y.chars())
                .all(|(xc, yc)| xc == 'o' || yc == 'o')
        })
        .count()
}

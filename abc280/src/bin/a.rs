use proconio::input;

fn main() {
    input! {
        h: u8,
        _: u8,
        ss: [String; h],
    }

    println!("{}", solve(&ss));
}

fn solve(ss: &[String]) -> impl std::fmt::Display {
    ss.iter()
        .map(|s| s.chars().filter(|&c| c == '#').count())
        .sum::<usize>()
}

use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
        t: Bytes,
    }

    println!("{}", solve(&s, &t));
}

fn solve(s: &[u8], t: &[u8]) -> impl std::fmt::Display {
    if t.starts_with(s) {
        "Yes"
    } else {
        "No"
    }
}

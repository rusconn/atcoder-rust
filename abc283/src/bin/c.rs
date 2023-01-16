use proconio::input;

fn main() {
    input! {
        s: u128,
    }

    println!("{}", solve(s));
}

fn solve(s: u128) -> impl std::fmt::Display {
    s.to_string().replace("00", "a").len()
}

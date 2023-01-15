use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    println!("{}", solve(n));
}

fn solve(n: i32) -> impl std::fmt::Display {
    n
}

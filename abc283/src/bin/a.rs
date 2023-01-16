use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    println!("{}", solve(a, b));
}

fn solve(a: u32, b: u32) -> impl std::fmt::Display {
    a.pow(b)
}

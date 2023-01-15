use proconio::input;

fn main() {
    input! {
        n: u32,
        ss: [String; n],
    }

    println!("{}", solve(ss));
}

fn solve(mut ss: Vec<String>) -> impl std::fmt::Display {
    ss.reverse();
    ss.join("\n")
}

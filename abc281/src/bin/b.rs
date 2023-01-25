use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!("{}", solve(&s));
}

fn solve(s: &str) -> impl std::fmt::Display {
    if sandwich(s) {
        "Yes"
    } else {
        "No"
    }
}

fn sandwich(s: &str) -> bool {
    if s.len() != 8 {
        return false;
    }

    let mut chars = s.chars();
    let head = chars.next().unwrap();
    let mut body = chars.collect::<String>();
    let tail = body.pop().unwrap();

    head.is_ascii_uppercase()
        && tail.is_ascii_uppercase()
        && body.parse::<u32>().map_or(false, |n| n >= 100000)
}

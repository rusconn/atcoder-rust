use maplit::hashset;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!("{}", solve(&s));
}

fn solve(s: &str) -> impl std::fmt::Display {
    if can(s) {
        "Yes"
    } else {
        "No"
    }
}

fn can(s: &str) -> bool {
    let mut box_ = hashset! {};
    let mut stack = vec![];
    let mut current = hashset! {};

    for c in s.chars() {
        match c {
            '(' => {
                stack.push(current);
                current = hashset! {};
            }
            ')' => {
                // in place な diff API が無い？
                for c2 in current {
                    box_.remove(&c2);
                }

                current = stack.pop().unwrap();
            }
            _ => {
                if box_.contains(&c) {
                    return false;
                }

                box_.insert(c);
                current.insert(c);
            }
        }
    }

    true
}

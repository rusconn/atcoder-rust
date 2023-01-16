use proconio::input;
use rustc_hash::FxHashSet;

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
    let mut box_ = FxHashSet::default();
    let mut stack = vec![];
    let mut current = FxHashSet::default();

    for c in s.chars() {
        match c {
            '(' => {
                stack.push(current);
                current = FxHashSet::default();
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

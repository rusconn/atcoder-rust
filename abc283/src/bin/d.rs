use std::collections::HashSet;

use maplit::hashset;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!("{}", solve(&s));
}

fn solve(s: &str) -> impl std::fmt::Display {
    yesno(can(s))
}

fn yesno(b: bool) -> &'static str {
    if b {
        "Yes"
    } else {
        "No"
    }
}

fn can(s: &str) -> bool {
    let mut box_: HashSet<char> = hashset! {};
    let mut stack: Vec<_> = vec![];
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

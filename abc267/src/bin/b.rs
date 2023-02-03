use std::fmt;

use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
    }

    println!("{}", solve(&s));
}

fn solve(s: &[u8]) -> impl fmt::Display {
    if is_split(s) {
        "Yes"
    } else {
        "No"
    }
}

fn is_split(s: &[u8]) -> bool {
    if s[0] == b'1' {
        return false;
    }

    let cols = [
        vec![s[6]],
        vec![s[3]],
        vec![s[1], s[7]],
        vec![s[0], s[4]],
        vec![s[2], s[8]],
        vec![s[5]],
        vec![s[9]],
    ];

    let mut a = false;
    let mut b = false;
    let mut c = false;

    for col in &cols {
        if !a {
            a = col.iter().any(|&b| b == b'1');
        } else if !b {
            b = col.iter().all(|&b| b == b'0');
        } else if !c {
            c = col.iter().any(|&b| b == b'1');
        }
    }

    a && b && c
}

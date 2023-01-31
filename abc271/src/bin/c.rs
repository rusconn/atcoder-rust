use std::fmt;

use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u32; n],
    }

    println!("{}", solve(a));
}

fn solve(mut a: Vec<u32>) -> impl fmt::Display {
    a.push(0);
    a.sort_unstable();

    let mut p = 0;
    let mut i = 1;
    let mut j = a.len() - 1;

    while i <= j {
        if p + 1 == a[i] {
            i += 1;
            p += 1;
            continue;
        }

        if j - i >= 1 && j >= 2 {
            j -= 2;
            p += 1;
            continue;
        }

        break;
    }

    p
}

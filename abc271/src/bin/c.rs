use std::fmt;

use proconio::input;

fn main() {
    input! {
        n: u32,
        mut a: [u32; n],
    }

    println!("{}", solve(&mut a));
}

fn solve(a: &mut [u32]) -> impl fmt::Display {
    a.sort_unstable();

    let mut p = 0;
    let mut i = 0;
    let mut j = a.len() - 1;

    while i <= j {
        if p + 1 == a[i] {
            i += 1;
            p += 1;
            continue;
        }

        if j - i >= 1 {
            j -= 2;
            p += 1;
            continue;
        }

        break;
    }

    p
}

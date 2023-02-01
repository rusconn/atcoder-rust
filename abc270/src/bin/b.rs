use std::fmt;

use proconio::input;

fn main() {
    input! {
        x: i16,
        y: i16,
        z: i16,
    }

    println!("{}", solve(x, y, z));
}

fn solve(mut x: i16, mut y: i16, mut z: i16) -> impl fmt::Display {
    if x < 0 {
        x = -x;
        y = -y;
        z = -z;
    }

    if y < 0 || x < y {
        x
    } else if y < z {
        -1
    } else if 0 < z {
        x
    } else {
        -2 * z + x
    }
}

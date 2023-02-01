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

fn solve(x: i16, y: i16, z: i16) -> impl fmt::Display {
    if x < 0 {
        if y < x || 0 < y {
            return x;
        }

        if z < y {
            return -1;
        }

        if z < 0 {
            return x;
        }

        2 * z - x
    } else {
        if y < 0 || x < y {
            return x;
        }

        if y < z {
            return -1;
        }

        if 0 < z {
            return x;
        }

        -2 * z + x
    }
}

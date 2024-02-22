use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: i32,
        p: i32,
        ps: [i32; n - 1],
    }

    println!("{}", cmp::max(0, ps.iter().max().unwrap_or(&0) - p + 1));
}

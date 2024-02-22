use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
        abs: [(i32, i32); m],
    }

    let mut s = (1..=n).collect::<BTreeSet<_>>();

    for (_, b) in abs {
        s.remove(&b);
    }

    let ans = if s.len() == 1 {
        s.pop_first().unwrap()
    } else {
        -1
    };

    println!("{ans}");
}

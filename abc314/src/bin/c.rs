use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        cs: [Usize1; n],
    }

    let mut vs = vec![VecDeque::new(); m];

    for (i, &c) in cs.iter().enumerate() {
        vs[c].push_back(s[i]);
    }

    for v in vs.iter_mut() {
        v.rotate_right(1);
    }

    let mut s = String::new();

    for c in cs {
        s.push(vs[c].pop_front().unwrap());
    }

    println!("{s}");
}

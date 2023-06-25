use std::fmt;

use proconio::input;

enum Quad {
    U,
    R,
    D,
    L,
}

fn main() {
    input! {
        r: usize,
        c: usize,
    }

    println!("{}", solve(r, c));
}

fn solve(r: usize, c: usize) -> impl fmt::Display {
    let q;

    if r <= c {
        q = if r + c <= 16 { Quad::U } else { Quad::R }
    } else {
        q = if r + c <= 16 { Quad::L } else { Quad::D }
    }

    let parity = match q {
        Quad::U => r % 2,
        Quad::R => (16 - c) % 2,
        Quad::D => (16 - r) % 2,
        Quad::L => c % 2,
    };

    if parity == 1 {
        "black"
    } else {
        "white"
    }
}

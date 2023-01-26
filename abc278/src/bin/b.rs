use std::fmt;

use proconio::input;

fn main() {
    input! {
        h: u8,
        m: u8,
    }

    println!("{}", solve(h, m));
}

fn solve(h: u8, m: u8) -> impl fmt::Display {
    let minutes = u16::from(h) * 60 + u16::from(m);
    let minutes = (minutes..).find(|&m| is_misleading(m)).unwrap();

    format!("{} {}", minutes / 60 % 24, minutes % 60)
}

fn is_misleading(minutes: u16) -> bool {
    let h = minutes / 60;
    let m = minutes % 60;

    let new_h = h / 10 * 10 + m / 10;
    let new_m = h % 10 * 10 + m % 10;

    new_h < 24 && new_m < 60
}

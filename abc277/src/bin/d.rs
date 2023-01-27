use std::fmt;

use proconio::input;

fn main() {
    input! {
        n: u32,
        m: u64,
        mut a: [u64; n],
    }

    println!("{}", solve(m, &mut a));
}

fn solve(m: u64, a: &mut [u64]) -> impl fmt::Display {
    a.sort_unstable();

    let mut sums = vec![];
    let mut sum = a[0];
    let mut prev = a[0];

    for &x in a.iter().skip(1) {
        if x == prev || x == prev + 1 {
            sum += x;
        } else {
            sums.push(sum);
            sum = x;
        }

        prev = x;
    }

    sums.push(sum);

    if sums.len() > 1 && (a[a.len() - 1] + 1) % m == a[0] {
        let sum = sums.pop().unwrap();
        sums[0] += sum;
    }

    sums.iter().sum::<u64>() - sums.into_iter().max().unwrap()
}

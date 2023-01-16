use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u32,
        mut a: [u32; n],
        q: u32,
    }

    println!("{}", solve(&mut a, q));
}

fn solve(a: &mut [u32], q: u32) -> impl std::fmt::Display {
    (0..q)
        .filter_map(|_| {
            input! {
                num: u32,
                k: usize,
            }

            if num == 1 {
                input! {
                    x: u32,
                }

                a[k - 1] = x;

                None
            } else {
                Some(a[k - 1])
            }
        })
        .join("\n")
}

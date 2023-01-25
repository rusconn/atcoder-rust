use proconio::input;

fn main() {
    input! {
        n: u32,
        t: u64,
        a: [u32; n],
    }

    println!("{}", solve(t, &a));
}

fn solve(t: u64, a: &[u32]) -> impl std::fmt::Display {
    let mut acc: u64 = 0;

    for (i, &x) in a.iter().enumerate().cycle() {
        acc += u64::from(x);

        if acc > t {
            return format!("{} {}", i + 1, u64::from(x) - (acc - t));
        }
    }

    unreachable!()
}

use proconio::input;

fn main() {
    input! {
        t: u32,
    }

    for _ in 0..t {
        input! {
            n: u32,
            as_: [u32; n],
        }

        println!("{}", solve(&as_));
    }
}

fn solve(as_: &[u32]) -> usize {
    as_.iter().filter(|&&a| is_odd(a)).count()
}

fn is_odd(x: u32) -> bool {
    x % 2 == 1
}

use proconio::input;

fn main() {
    input! {
        n: i32,
        t: i64,
        a: [i64; n],
    }

    println!("{}", solve(t, &a));
}

fn solve(t: i64, a: &[i64]) -> impl std::fmt::Display {
    let cycle = a.iter().sum::<i64>();
    let mut rest = t % cycle;

    for (i, &x) in a.iter().enumerate() {
        if rest < x {
            return format!("{} {}", i + 1, rest);
        } else {
            rest -= x;
        }
    }

    unreachable!()
}

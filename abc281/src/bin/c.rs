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
    let mut i = 1;

    for x in a.iter() {
        if (rest - x) > 0 {
            rest -= x;
            i += 1;
        } else {
            break;
        }
    }

    format!("{} {}", i, rest)
}

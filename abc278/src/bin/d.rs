use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
        mut a: [u64; n],
        q: u32,
    }

    let mut answers = vec![];

    for _ in 0..q {
        input! {
            kind: u8,
        }

        match kind {
            1 => {
                input! {
                    x: u64,
                }

                for ax in &mut a {
                    *ax = x;
                }
            }
            2 => {
                input! {
                    i: usize,
                    x: u64,
                }

                a[i - 1] += x;
            }
            _ => {
                input! {
                    i: usize,
                }

                answers.push(a[i - 1]);
            }
        }
    }

    for a in answers {
        println!("{}", a);
    }
}

use proconio::{fastout, input};
use rustc_hash::FxHashMap;

#[fastout]
fn main() {
    input! {
        n: u32,
        mut a: [u64; n],
        q: u32,
    }

    let mut assign: Option<u64> = None;
    let mut adds = FxHashMap::default();
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

                assign = Some(x);
                adds.clear();
            }
            2 => {
                input! {
                    i: usize,
                    x: u64,
                }

                *adds.entry(i - 1).or_insert(0) += x;
            }
            _ => {
                input! {
                    i: usize,
                }

                let add = adds.get(&(i - 1)).unwrap_or(&0);

                if let Some(m) = assign {
                    answers.push(m + add);
                } else {
                    answers.push(a[i - 1] + add);
                }
            }
        }
    }

    for a in answers {
        println!("{}", a);
    }
}

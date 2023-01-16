use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
        mut a: [u32; n],
        q: u32,
    }

    for _ in 0..q {
        input! {
            num: u32,
            k: usize,
        }

        if num == 1 {
            input! {
                x: u32,
            }

            a[k - 1] = x;
        } else {
            println!("{}", a[k - 1]);
        }
    }
}

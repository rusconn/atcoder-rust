use proconio::{fastout, input};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: u8,
        mut ps: [u8; n],
    }

    ps.prev_permutation();

    for p in ps {
        print!("{} ", p);
    }
}

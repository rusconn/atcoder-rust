use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ss: [String; n],
    }

    for i in 0..=(n - 9) {
        for j in 0..=(m - 9) {
            if is_tak_code(&ss, (i, j)) {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
}

#[inline]
fn is_tak_code(ss: &[String], (i, j): (usize, usize)) -> bool {
    [(0, "###."), (1, "###."), (2, "###."), (3, "....")]
        .iter()
        .all(|(o, s)| ss[i + o][j..j + 4] == **s)
        && [(5, "...."), (6, ".###"), (7, ".###"), (8, ".###")]
            .iter()
            .all(|(o, s)| ss[i + o][j + 5..j + 9] == **s)
}

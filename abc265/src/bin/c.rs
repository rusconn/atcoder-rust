use std::fmt;

use proconio::{input, marker::Bytes};

fn main() {
    input! {
        h: usize,
        w: usize,
        gss: [Bytes; h],
    }

    println!("{}", solve(h, w, &gss));
}

fn solve(h: usize, w: usize, gss: &[Vec<u8>]) -> impl fmt::Display {
    if let Some((i, j)) = do_move(h, w, gss) {
        format!("{} {}", i + 1, j + 1)
    } else {
        "-1".to_string()
    }
}

fn do_move(h: usize, w: usize, gss: &[Vec<u8>]) -> Option<(usize, usize)> {
    let mut visited = vec![vec![false; w]; h];

    let mut i = 0;
    let mut j = 0;

    while !visited[i][j] {
        visited[i][j] = true;

        match gss[i][j] {
            b'U' => {
                if i == 0 {
                    return Some((i, j));
                }

                i -= 1;
            }
            b'D' => {
                if i == h - 1 {
                    return Some((i, j));
                }

                i += 1;
            }
            b'L' => {
                if j == 0 {
                    return Some((i, j));
                }

                j -= 1;
            }
            _ => {
                if j == w - 1 {
                    return Some((i, j));
                }

                j += 1;
            }
        }
    }

    None
}

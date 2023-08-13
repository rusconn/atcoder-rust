use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        mut s: Chars,
        q: usize,
        txcs: [(u8, usize, char); q],
    }

    let i = txcs.iter().rposition(|x| x.0 != 1);
    let t = i.map(|i| txcs[i].0).unwrap_or(1);
    let i = i.unwrap_or(0);

    let (first, second) = txcs.split_at(i);

    for &(_, c, x) in first.iter().filter(|x| x.0 == 1) {
        s[c - 1] = x;
    }

    if t == 2 {
        s.iter_mut().for_each(|c| *c = c.to_ascii_lowercase());
    }
    if t == 3 {
        s.iter_mut().for_each(|c| *c = c.to_ascii_uppercase());
    };

    for &(_, c, x) in second.iter().filter(|x| x.0 == 1) {
        s[c - 1] = x;
    }

    println!("{}", s.iter().collect::<String>());
}

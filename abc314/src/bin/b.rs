use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut a_i = vec![vec![]; 37];
    let mut bets = vec![38; n];

    for (i, bet) in bets.iter_mut().enumerate() {
        input! {
            c: usize,
            as_: [usize; c],
        }

        for a in as_ {
            a_i[a].push(i);
        }

        *bet = c;
    }

    input! {
        x: usize
    };

    let is = &a_i[x];
    let mins = is.iter().min_set_by(|&&i, &&j| bets[i].cmp(&bets[j]));

    println!("{}", mins.len());
    println!("{}", mins.iter().map(|&a| a + 1).join(" "));
}

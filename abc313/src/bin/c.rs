use proconio::input;

fn main() {
    input! {
        n: usize,
        ai: [usize; n],
    }

    let sum: usize = ai.iter().sum();
    let means = sum / n;
    let meanl = means + 1;
    let x: usize = ai.iter().filter(|&&a| a < means).map(|a| means - a).sum();
    let y: usize = ai.iter().filter(|&&a| a > meanl).map(|a| a - meanl).sum();

    println!("{}", x.max(y));
}

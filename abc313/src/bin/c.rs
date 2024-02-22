use proconio::input;

fn main() {
    input! {
        n: usize,
        ai: [usize; n],
    }

    let sum: usize = ai.iter().sum();
    let mean = sum as f64 / n as f64;
    let (meanf, meanc) = (mean.floor() as usize, mean.ceil() as usize);
    let x: usize = ai.iter().filter(|&&a| a < meanf).map(|a| meanf - a).sum();
    let y: usize = ai.iter().filter(|&&a| a > meanc).map(|a| a - meanc).sum();

    println!("{}", x.max(y));
}

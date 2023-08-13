use proconio::input;

const PI: &str = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";

fn main() {
    input! {
        n: usize,
    }

    let ans = &PI[..n + 2];

    println!("{ans}");
}

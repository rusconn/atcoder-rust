use proconio::input;

const CHORD: [&str; 7] = ["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"];

fn main() {
    input! {
        s: String,
    }

    let ans = if CHORD.contains(&s.as_str()) {
        "Yes"
    } else {
        "No"
    };

    println!("{ans}");
}

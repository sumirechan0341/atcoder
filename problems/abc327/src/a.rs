use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        s: String
    };
    println!("{}", if s.contains("ab") || s.contains("ba") {"Yes"} else { "No" });
}
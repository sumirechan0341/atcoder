use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: String,
        t: String
    };
    println!("{}", if s.contains(&t) { "Yes" } else { "No" });
}
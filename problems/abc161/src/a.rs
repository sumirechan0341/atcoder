use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    println!("{}", if s[2] == s[3] && s[4] == s[5] { "Yes" } else { "No" });
}
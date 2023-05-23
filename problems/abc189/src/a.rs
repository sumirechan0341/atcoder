use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    println!("{}", if s[0] == s[1] && s[1] == s[2] { "Won" } else { "Lost" });
}
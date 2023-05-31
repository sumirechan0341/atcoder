use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    println!("{}", if s[s.len()-1] == 'T' { "YES" } else { "NO" });
}
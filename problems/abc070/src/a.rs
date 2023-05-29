use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: Chars
    };
    println!("{}", if n[0] == n[2] { "Yes" } else { "No" });
}
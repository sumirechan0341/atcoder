use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        c1: Chars,
        c2: Chars
    };
    println!("{}", if c1[0] == c2[2] && c1[2] == c2[0] && c1[1] == c2[1] { "YES" } else { "NO" });
}
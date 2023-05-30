use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        _a: String,
        s: Chars,
        _c: String
    };
    println!("A{}C", s[0]);
}
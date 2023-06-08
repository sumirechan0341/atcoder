use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        s: Chars
    };
    println!("{}", "x".repeat(s.len()));
}
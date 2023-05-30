use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        s: Chars,
        i: usize
    };
    println!("{}", s[i-1]);
}
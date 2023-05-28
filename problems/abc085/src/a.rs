use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        mut s: Chars
    };
    s[3] = '8';
    println!("{}", s.iter().collect::<String>());
}
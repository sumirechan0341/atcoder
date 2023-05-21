use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        s: String
    };
    println!("{}", s[1..3].to_string() + &s[0..1]);
}
use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        x: char,
        y: char
    };
    println!("{}", if x < y { "<" } else if x == y { "=" } else { ">" });
}
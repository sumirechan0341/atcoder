use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: i32,
        x: i32
    };
    println!("{}", (x-1).min(n-x));
}
use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: i64,
        k: i64,
        x: i64,
        y: i64
    };
    println!("{}", n.min(k) * x + (n-k).max(0) * y);
}
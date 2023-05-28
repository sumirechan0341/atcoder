use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: i32,
        a: i32,
        b: i32
    };
    println!("{}", b.min(n * a));
}
use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: i32,
        a: i32
    };
    println!("{}", n * n - a);
}
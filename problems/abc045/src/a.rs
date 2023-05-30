use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        a: i32,
        b: i32,
        h: i32
    };
    println!("{}", (a + b) * h / 2);
}
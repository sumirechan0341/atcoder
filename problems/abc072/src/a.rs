use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        x: i32,
        t: i32
    };
    println!("{}", 0.max(x-t));
}
use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        s: i32,
        t: i32
    };
    println!("{}", t - s + 1);
}
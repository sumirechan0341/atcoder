use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        x: i32,
        a: i32,
        b: i32
    };
    println!("{}", if (x - a).abs() > (x - b).abs() { "B" } else { "A" });
}
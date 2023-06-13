use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        x: i32,
        y: i32,
        z: i32
    };
    println!("{}", (x-z)/(y+z));
}
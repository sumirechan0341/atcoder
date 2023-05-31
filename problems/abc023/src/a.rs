use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        x: i32
    };
    println!("{}", x / 10 + x % 10);
}
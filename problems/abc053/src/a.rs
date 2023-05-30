use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        x: i32
    };
    println!("{}", if x < 1200 { "ABC" } else { "ARC" });
}
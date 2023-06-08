use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        x: i32,
        y: i32
    };
    println!("{}", if 2 * x <= y && y <= 4 * x && x % 2 == 0 { "Yes" } else { "No" });
}
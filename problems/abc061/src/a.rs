use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32
    };
    println!("{}", if a <= c && c <= b { "Yes" } else { "No" });
}
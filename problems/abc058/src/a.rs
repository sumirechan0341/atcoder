use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32
    };
    println!("{}", if b - a == c - b {"YES"} else { "NO" });
}
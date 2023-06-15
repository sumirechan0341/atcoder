use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h1: i32,
        w1: i32,
        h2: i32,
        w2: i32
    };
    println!("{}", if h1 == h2 || h1 == w2 || h2 == w1 || w1 == w2 { "YES" } else { "NO" });
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        m: i32,
        d: i32
    };
    println!("{}", if m % d == 0 { "YES" } else { "NO" });
}
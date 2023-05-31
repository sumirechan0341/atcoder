use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: i32
    };
    println!("{}", if n % 3 == 0 { "YES" } else { "NO" });
}
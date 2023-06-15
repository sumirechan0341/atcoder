use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64
    };
    let p = 1000000007;
    println!("{}", ((a * b) % p * c) % p);
}
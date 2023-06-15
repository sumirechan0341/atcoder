use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: u32,
        k: u64
    };
    println!("{}", k*(k-1).pow(n-1));
}
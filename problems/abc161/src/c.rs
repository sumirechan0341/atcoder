use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i64,
        k: i64
    };
    println!("{}", (n%k).min((n%k-k).abs()));
}
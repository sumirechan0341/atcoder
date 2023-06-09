use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i64,
        b: i64,
        k: i64
    };
    println!("{} {}", (a-k).max(0), (b+((a-k).min(0))).max(0));
}
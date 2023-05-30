use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32,
        m: i32
    };
    println!("{}", (n - 1) * (m - 1));
}
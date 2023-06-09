use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32,
        d: i32
    };
    println!("{}", (n + (2*d+1) - 1) / (2*d+1));
}
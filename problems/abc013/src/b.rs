use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32,
        b: i32
    };
    println!("{}", (a-b).abs().min(10-(a-b).abs()));
}
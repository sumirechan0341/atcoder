use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32,
        b: i32
    };
    println!("{}", (b + a - 1) / a);
}
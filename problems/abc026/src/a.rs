use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32
    };
    println!("{}", (a / 2).pow(2));
}
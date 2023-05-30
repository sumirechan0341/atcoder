use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32
    };
    println!("{}", n * 800 - n / 15 * 200)
}
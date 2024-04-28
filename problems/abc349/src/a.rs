use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an1: [i32; n-1]
    };
    println!("{}", -an1.iter().sum::<i32>());
}

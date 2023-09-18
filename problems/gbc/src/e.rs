use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i64
    };
    if n == 2 {
        println!("{}", 1);
    } else {
        println!("{}", 2);
    }
}
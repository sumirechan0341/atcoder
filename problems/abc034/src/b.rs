use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32
    };
    println!("{}", if n % 2 == 0 { n-1 } else { n+1 });
}
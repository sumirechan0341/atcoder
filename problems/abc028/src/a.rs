use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32        
    };
    println!("{}", if n == 100 { "Perfect" } else if n >= 90 { "Great" } else if n >= 60 { "Good" } else { "Bad" });
}
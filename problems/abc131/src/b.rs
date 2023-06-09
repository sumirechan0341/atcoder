use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32,
        l: i32
    };
    if l < 0 && n + l > 0 {
        println!("{}", (l..n+l).sum::<i32>());
        return;
    }
    if n + l <= 0 {
        println!("{}", (l..n+l-1).sum::<i32>());
        return;
    }
    println!("{}", (l+1..n+l).sum::<i32>());
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut n: u64,
        k: u64
    };
    for _ in 0..k {
        if n % 200 == 0 {
            n /= 200;
        } else {
            n = n * 1000 + 200;
        }
    }
    println!("{}", n);
}
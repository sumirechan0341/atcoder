use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut count = 0;
    for i in 0..n-1 {
        if s[i] == s[i+1] {
            count += 1;
        }
    }
    println!("{}", n-count);
}
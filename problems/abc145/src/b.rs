use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        s: Chars
    };
    if s.len() % 2 == 1 {
        println!("{}", "No");
        return;
    }
    for i in 0..n/2 {
        if s[i] != s[i+n/2] {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}
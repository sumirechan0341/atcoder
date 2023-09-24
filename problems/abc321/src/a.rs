use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: Chars
    };
    let mut prev = 10;
    for i in 0..n.len() {
        if n[i].to_digit(10).unwrap() >= prev {
            println!("{}", "No");
            return;
        }
        prev = n[i].to_digit(10).unwrap()
    }
    println!("{}", "Yes");
}
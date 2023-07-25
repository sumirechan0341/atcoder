use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    println!("{}", 2*s.iter().filter(|&x| *x != '1').count().min(s.iter().filter(|&x| *x != '0').count()));
}
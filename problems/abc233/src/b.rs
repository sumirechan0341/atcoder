use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        l: usize,
        r: usize,
        s: Chars
    };
    let s1 = s[0..l-1].into_iter().collect::<Vec<_>>();
    let s2 = s[l-1..r].into_iter().rev().collect::<Vec<_>>();
    let s3 = s[r..].into_iter().collect::<Vec<_>>();
    println!("{}", vec![s1, s2, s3].concat().into_iter().collect::<String>());
 
}
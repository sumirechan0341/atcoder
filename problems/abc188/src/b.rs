use proconio::{input, marker::Chars};
use itertools::zip;
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        an: [i32; n],
        bn: [i32; n]
    };
    println!("{}", if zip(an, bn).map(|(a, b)| a*b).collect::<Vec<_>>().iter().sum::<i32>() == 0 { "Yes" } else { "No" });
} 
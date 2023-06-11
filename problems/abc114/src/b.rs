use proconio::{input, marker::Chars};
use itertools::Itertools;
use std::i32::MAX;
type VS = Vec<String>;

pub fn main() {
    input !{
        s: Chars
    };
    let mut min = MAX;
    for i in 0..s.len()-2 {
        let n = (s[i].to_digit(10).unwrap() * 100 + s[i+1].to_digit(10).unwrap() * 10 + s[i+2].to_digit(10).unwrap()) as i32;
        if min > (n - 753).abs() {
            min = (n - 753).abs() as i32;
        }
    }
    println!("{}", min);
}
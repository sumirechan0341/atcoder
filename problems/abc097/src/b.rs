use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        x: i32
    };
    let mut d = vec![];
    d.push(1);
    for i in 2..33 {
        let mut b = i*i;
        while b <= 1000 {
            d.push(b);
            b *= i;
        }
    }
    d.sort();
    println!("{}", d.into_iter().filter(|y| y <= &x).last().unwrap());
}
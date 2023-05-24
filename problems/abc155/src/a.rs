use proconio::input;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn main() {
    input !{
        abc: [i32; 3]
    };
    println!("{}", if HashSet::<i32>::from_iter(abc).len() == 2 { "Yes" } else { "No" });
}
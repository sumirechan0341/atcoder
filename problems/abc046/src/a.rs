use proconio::{input, marker::Chars};
use std::collections::HashSet;
use std::iter::FromIterator;
pub fn main() {
    input !{
        abc: [i32; 3]
    };
    println!("{}", HashSet::<i32>::from_iter(abc).len());
}
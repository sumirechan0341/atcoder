use std::collections::HashSet;
use std::iter::FromIterator;
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        an: [i32; n]
    };
    let ans = HashSet::<i32>::from_iter(an).len();
    println!("{}", ans);
}
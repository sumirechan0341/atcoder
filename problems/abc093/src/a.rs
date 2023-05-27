use proconio::{input, marker::Chars};
use std::collections::HashSet;
use std::iter::FromIterator;
pub fn main() {
    input !{
        c: Chars        
    };
    println!("{}", if HashSet::<char>::from_iter(c).len() == 3 { "Yes" } else { "No" });
}
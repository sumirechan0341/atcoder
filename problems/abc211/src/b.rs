use std::collections::HashSet;
use std::iter::FromIterator;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        ss: [String; 4]
    };
    println!("{}", if HashSet::<String>::from_iter(ss).len() == 4 { "Yes" } else { "No" });    
}
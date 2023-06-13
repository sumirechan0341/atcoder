use std::collections::HashSet;
use std::iter::FromIterator;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        sn: [String; n]
    };
    let ans = HashSet::<String>::from_iter(sn);
    println!("{}", if ans.len() == 4 { "Four" } else { "Three" });
}
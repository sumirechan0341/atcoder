use std::collections::HashSet;
use std::iter::FromIterator;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    let ss = HashSet::<char>::from_iter(s.clone());
    println!("{}", if s.len() == ss.len() { "yes" } else { "no" });
}
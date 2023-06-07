use std::collections::HashSet;
use std::iter::FromIterator;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i32; n]
    };
    println!("{}", if HashSet::<i32>::from_iter(an.clone()).len() == an.len() { "Yes" } else { "No" });
}
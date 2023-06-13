use std::collections::HashSet;
use std::iter::FromIterator;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        dn: [i32; n]
    };
    let mochi = HashSet::<i32>::from_iter(dn);
    println!("{}", mochi.len());
}
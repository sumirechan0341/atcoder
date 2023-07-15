use std::collections::HashSet;
use std::iter::FromIterator;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        sn: [String; n]
    };
    let set = HashSet::<String>::from_iter(sn);
    println!("{}", set.len());
}
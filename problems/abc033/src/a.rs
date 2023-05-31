use proconio::{input, marker::Chars};
use std::collections::HashSet;
use std::iter::FromIterator;
pub fn main() {
    input! {
        n: Chars
    };
    println!("{}", if HashSet::<char>::from_iter(n).len() == 1 { "SAME" } else { "DIFFERENT" });
}
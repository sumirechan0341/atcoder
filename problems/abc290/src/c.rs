use proconio::{input, marker::Chars};

use std::collections::HashSet;
use std::iter::FromIterator;
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i32,
        k: usize,
        an: [usize; n]
    };
    let anset = HashSet::<usize>::from_iter(an.clone());
    let mut sorted_anset = anset.into_iter().collect::<Vec<_>>();
    sorted_anset.sort();
    for i in 0..sorted_anset.len().min(k) {
        if sorted_anset[i] != i {
            println!("{}", i);
            return
        }
    }
    println!("{}", sorted_anset.len().min(k));
}
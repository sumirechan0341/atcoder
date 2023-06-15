use proconio::{input, marker::Chars};
use std::collections::HashSet;
use std::iter::FromIterator;
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        a: usize,
        b: usize,
        k: usize,
        pk: [usize; k]        
    };
    let pset = HashSet::<usize>::from_iter(pk.clone());
    println!("{}", if !pk.contains(&a) && !pk.contains(&b) && pset.len() == pk.len() { "YES" } else { "NO" });
}
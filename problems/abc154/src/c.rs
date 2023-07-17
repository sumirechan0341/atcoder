use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i32; n]
    };
    let mut set = HashSet::<i32>::new();
    for i in 0..n {
        set.insert(an[i]);
    }
    println!("{}", if an.len() == set.len() { "YES" } else { "NO" });
}
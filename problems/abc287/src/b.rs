use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        ss: [String; n],
        ts: [String; m]
    };
    let mut tset = HashSet::<String>::new();
    for t in ts {
        tset.insert(t);
    }
    println!("{}", ss.iter().filter(|s| tset.contains(&s[s.len()-3..s.len()])).collect::<Vec<&String>>().len());
}
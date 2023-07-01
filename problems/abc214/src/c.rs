use std::collections::BTreeSet;

use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        mut s: Chars,
        k: usize
    };
    s.sort();
    let mut ans = BTreeSet::<String>::new();
    for c in (0..s.len()).permutations(s.len()) {
        let mut local = vec![];
        for i in c {
            local.push(s[i]);
        }
        ans.insert(local.iter().collect::<String>());
    }
    println!("{}", ans.iter().nth(k-1).unwrap()); 
}
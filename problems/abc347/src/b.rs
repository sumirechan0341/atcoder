use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    let mut ans = HashSet::new();
    for i in 0..s.len() {
        for j in i..s.len() {
            ans.insert(s[i..=j].to_vec());
        }
    }
    println!("{}", ans.len());
}

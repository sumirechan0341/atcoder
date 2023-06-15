use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars,
        k: i32
    };
    let mut ans = HashSet::<String>::new();
    if k as usize > s.len() {
        println!("{}", 0);
        return ;
    }
    for i in 0..=s.len()-k as usize {
        ans.insert(s[i..i+k as usize].into_iter().collect::<String>());
    }
    println!("{}", ans.len());
}
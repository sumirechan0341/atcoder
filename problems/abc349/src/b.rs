use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    let mut v = vec![0; 26];
    for i in 0..s.len() {
        v[s[i] as usize - 'a' as usize] += 1;
    }
    let mut cnt = vec![0; s.len() + 1];
    for i in 0..26 {
        cnt[v[i]] += 1;
    }
    println!(
        "{}",
        if cnt[1..].iter().all(|&x| x == 0 || x == 2) {
            "Yes"
        } else {
            "No"
        }
    );
}

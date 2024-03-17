use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    let mut c = vec![0; 26];
    for i in 0..s.len() {
        c[(s[i] as u8 - 'a' as u8) as usize] += 1;
    }
    let mut ans = s.len() * (s.len() - 1) / 2;
    let mut flag = false;
    for i in 0..26 {
        if c[i] != 0 {
            ans -= c[i] * (c[i] - 1) / 2;
        }
        if c[i] > 1 {
            flag = true
        }
    }
    if flag {
        ans += 1;
    }
    println!("{}", ans);
}

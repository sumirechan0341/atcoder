use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut an: [usize; n]
    };
    let mut t = vec![0; n];
    for i in 0..n {
        t[an[i] - 1] = i;
    }

    let mut ans = vec![];
    for i in 0..n {
        if an[i] != i + 1 {
            ans.push((i + 1, t[i] + 1));
            an[t[i]] = an[i];
            t[an[i] - 1] = t[i];
        }
    }
    println!("{}", ans.len());
    for i in 0..ans.len() {
        println!("{} {}", ans[i].0, ans[i].1);
    }
}

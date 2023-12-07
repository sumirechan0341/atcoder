use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n]
    };
    let mut ans = n * (n - 1) * (n - 2) / 6;
    let mut cnt = vec![0; 200001];
    for i in 0..n {
        cnt[an[i]] += 1;
    }
    for i in 1..200001 {
        if cnt[i] > 1 {
            ans -= cnt[i] * (cnt[i] - 1) / 2 * (n - cnt[i]);
        }
        if cnt[i] > 2 {
            ans -= cnt[i] * (cnt[i] - 1) * (cnt[i] - 2) / 6;
        }
    }
    println!("{}", ans);
}

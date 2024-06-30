use std::collections::BinaryHeap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n],
        wn: [usize; n]
    };
    let mut ans = 0;
    let mut v = vec![0; n];
    for i in 0..n {
        ans += v[an[i] - 1].min(wn[i]);
        v[an[i] - 1] = v[an[i] - 1].max(wn[i]);
    }
    println!("{}", ans);
}

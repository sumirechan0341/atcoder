use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut a: [i64; n]
    };
    let mut pq = BinaryHeap::new();
    let mut deg = vec![1; n];
    let mut ans = 0;
    for i in 0..n {
        pq.push(Reverse(((2 * deg[i] + 1) * a[i], i)));
        ans += a[i];
    }
    for _i in 0..n - 2 {
        let Reverse((min, idx)) = pq.pop().unwrap();
        ans += min;
        deg[idx] += 1;
        pq.push(Reverse(((deg[idx] * 2 + 1) * a[idx], idx)));
    }
    println!("{}", ans);
}

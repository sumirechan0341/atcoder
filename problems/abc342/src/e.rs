use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use proconio::{input, marker::Chars};
// ダイクストラ実装例
pub fn main() {
    input! {
        n: usize,
        m: usize,
        ldkcabm: [(i128, i128, i128, i128, usize, usize); m]
    };
    let mut graph = vec![vec![]; n];
    for &(l, d, k, c, a, b) in &ldkcabm {
        graph[b - 1].push((l + d * (k - 1), a - 1));
    }
    let mut cost = vec![i128::MIN / 2; n];
    cost[n - 1] = i128::MAX / 2;
    let mut pq = BinaryHeap::new();
    pq.push((i128::MAX / 2, n - 1));
    while let Some((c, v)) = pq.pop() {
        if cost[v] > c {
            continue;
        }
        for &(next_c, next_v) in &graph[v] {
            // next_cはnext_vにおける終電時間
            let (l, d, k, c1, a, b) = ldkcabm[next_v];
            // 終電時間を超えて到着しないようにする
            let t = l + (cost[v] + c1 - l) / d * d;
            if l > c {
                continue;
            }
            if next_c.min(t) > cost[next_v] {
                cost[next_v] = next_c.min(t);

                pq.push((next_c.min(t), next_v));
            }
        }
    }
    for i in 0..n - 1 {
        if cost[i] == i128::MIN / 2 {
            println!("{}", "Unreachable");
        } else {
            println!("{}", cost[i]);
        }
    }
}

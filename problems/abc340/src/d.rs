use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use proconio::{input, marker::Chars};
// ダイクストラ実装例
pub fn main() {
    input! {
        n: usize,
        abxn1: [(usize, usize, usize); n-1]
    };
    let mut graph = vec![vec![]; n];
    for i in 0..n - 1 {
        let (a, b, x) = abxn1[i];
        graph[i].push((a, i + 1));
        graph[i].push((b, x - 1));
    }
    let mut cost = vec![usize::MAX / 2; n];
    cost[0] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, 0)));
    while let Some(Reverse((c, v))) = pq.pop() {
        if cost[v] < c {
            continue;
        }
        for &(next_c, next_v) in &graph[v] {
            let next_c = c + next_c;
            if next_c < cost[next_v] {
                cost[next_v] = next_c;
                pq.push(Reverse((next_c, next_v)));
            }
        }
    }
    println!("{}", cost[n - 1]);
}

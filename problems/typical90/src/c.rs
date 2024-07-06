use std::collections::{BinaryHeap, VecDeque};

use itertools::Itertools;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        abn1: [(usize, usize); n-1]
    };
    // 頂点数N、辺の数がN-1本
    // すべての頂点が連結
    // => グラフは木
    let mut graph = vec![vec![]; n];
    for i in 0..n - 1 {
        graph[abn1[i].0 - 1].push(abn1[i].1 - 1);
        graph[abn1[i].1 - 1].push(abn1[i].0 - 1);
    }
    let mut cost = vec![0; n];
    let mut queue = VecDeque::new();
    // 適当な始点からdfsして一番距離の遠い点を見つける
    queue.push_back(0);
    while let Some(v) = queue.pop_front() {
        for &next in &graph[v] {
            if cost[next] == 0 {
                queue.push_front(next);
                cost[next] = cost[v] + 1;
            }
        }
    }
    // 一番遠かった点から一番遠い点が木の直径になる
    queue = VecDeque::new();
    queue.push_back(cost.iter().position_max().unwrap());
    let mut cost = vec![0; n];
    while let Some(v) = queue.pop_front() {
        for &next in &graph[v] {
            if cost[next] == 0 {
                queue.push_front(next);
                cost[next] = cost[v] + 1;
            }
        }
    }
    println!("{}", cost.iter().max().unwrap() + 1);
}

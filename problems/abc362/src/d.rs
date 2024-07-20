use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        an: [usize; n],
        uvbm: [(usize, usize, usize); m]
    };
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        graph[uvbm[i].0 - 1].push((uvbm[i].1 - 1, uvbm[i].2));
        graph[uvbm[i].1 - 1].push((uvbm[i].0 - 1, uvbm[i].2));
    }
    let mut d = vec![usize::MAX / 2; n];
    d[0] = an[0];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(an[0]), 0));
    while let Some((Reverse(cost), v)) = heap.pop() {
        if cost > d[v] {
            continue;
        }
        for &(u, ecost) in &graph[v] {
            if cost + ecost + an[u] < d[u] {
                d[u] = cost + ecost + an[u];
                heap.push((Reverse(d[u]), u));
            }
        }
    }
    for i in 1..n {
        println!("{}", d[i]);
    }
}

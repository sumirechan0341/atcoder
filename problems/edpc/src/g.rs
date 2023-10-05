use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        xym: [(usize, usize); m]
    };
    let mut graph = vec![vec![]; n+1];
    for &(x, y) in &xym {
        graph[x].push(y);
    }
    let dist = &mut vec![-1; n+1];
    for i in 1..=n {
        dfs(&graph, dist, i);
    }
    println!("{}", dist.iter().max().unwrap());
}

fn dfs(graph: &Vec<Vec<usize>>, dist: &mut Vec<i32>, current: usize) -> i32 {
    if dist[current] > 0 {
        return dist[current];
    }
    let mut max = -1;
    for &next in &graph[current] {
        let d = dfs(graph, dist, next);
        if d > max {
            max = d;
        }
    }
    dist[current] = max+1;
    return max+1;
}
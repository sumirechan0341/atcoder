use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m]
    };
    let mut graph = vec![vec![]; n+1];
    for (a, b) in abm {
        graph[a].push(b);
        graph[b].push(a);
    }
    let paths = &mut vec![];
    for i in 1..=n {
        dfs(i, graph.clone(), vec![], vec![], n, paths);
    }
    println!("{:?}", paths);
    println!("{}", paths.len());
}

fn dfs(start: usize, graph: Vec<Vec<usize>>, this_path: Vec<usize>, used: Vec<usize>, n: usize, paths: &mut Vec<Vec<(usize, usize)>>) {
    if used.contains(&start) {
        return
    }
    let mut local_path = this_path.clone();
    local_path.push(start);
    let mut sorted_local_path = vec![];
    if local_path.len() == n {
        let index1 = local_path.iter().position(|&x| x == 1).unwrap();
        println!("{}", index1);
        println!("{:?}", local_path);
        for i in 0..n {
            sorted_local_path.push(local_path[(i+index1)%n]);
        }
        if !paths.contains(&sorted_local_path) {
            paths.push(sorted_local_path);
        }
        return;
    }
    let mut new_used = used.clone();
    new_used.push(start);
    for next in &graph[start] {
        if used.contains(next) {
            continue;
        }
        dfs(*next, graph.clone(), local_path.clone(), new_used.clone(), n, paths);
    }
}
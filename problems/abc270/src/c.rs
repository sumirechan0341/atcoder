use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        uvn: [(usize, usize); n-1]
    };
    let mut graph = vec![vec![]; n];
    for (u, v) in uvn {
        graph[u-1].push(v-1);
        graph[v-1].push(u-1);
    }
    let used = &mut vec![false; n];
    let path = &mut VecDeque::<usize>::new();
    dfs(x-1, y-1, &graph, used, path);
    println!("{}", path.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}

fn dfs(current: usize, goal: usize, graph: &Vec<Vec<usize>>, used: &mut Vec<bool>, path: &mut VecDeque<usize>) -> bool {
    used[current] = true;
    path.push_back(current+1);
    if current == goal {
        return true;
    }
    
    for &next in &graph[current] {
        if !used[next] {
            used[next] = true;
            if dfs(next, goal, graph, used, path) {
                return true;
            }
        }
    }
    path.pop_back();
    return false;
}
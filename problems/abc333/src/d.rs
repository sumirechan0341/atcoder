use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        uvn: [(usize, usize); n-1]
    };
    let mut graph = vec![vec![]; n + 1];
    let mut queue = VecDeque::new();
    for i in 0..n - 1 {
        let (u, v) = uvn[i];
        graph[u].push(v);
        graph[v].push(u);
        if u == 1 {
            queue.push_back(v);
        }
        if v == 1 {
            queue.push_back(u);
        }
    }
    if queue.len() == 1 {
        println!("{}", 1);
        return;
    }
    let mut ans = vec![];
    let used = &mut vec![false; n + 1];
    used[1] = true;
    for i in queue {
        used[i] = true;
        let res = dfs(&graph, i, used);
        ans.push(res);
    }

    println!(
        "{}",
        ans.iter().sum::<usize>() - ans.iter().max().unwrap() + 1
    );
}
fn dfs(graph: &Vec<Vec<usize>>, next: usize, used: &mut Vec<bool>) -> usize {
    if graph[next].len() == 1 {
        return 1;
    }
    let mut res = 1;
    for &v in &graph[next] {
        if used[v] {
            continue;
        }
        used[v] = true;
        res += dfs(graph, v, used);
    }
    res
}

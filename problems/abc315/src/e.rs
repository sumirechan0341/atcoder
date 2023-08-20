use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
    };
    let mut graph = vec![vec![]; n+1];
    for i in 0..n {
        input!{
            c: usize,
            pc: [usize; c]
        };
        graph[i+1] = pc;
    }
    let ans = &mut vec![];
    let used = &mut  vec![false; n+1];
    dfs(&graph, 1, used, ans);
    println!("{}", ans[..ans.len()-1].join(" "));
}

fn dfs(graph: &Vec<Vec<usize>>, current: usize, used: &mut Vec<bool>, path: &mut Vec<String>) -> bool {
    if used[current] {
        return true;
    }
    used[current] = true;
    for &next in &graph[current] {
        if used[next] {
            continue;
        }
        dfs(graph, next, used, path);
    }
    path.push(current.to_string());
    return true;
}
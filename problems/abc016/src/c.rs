use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        abm: [(usize, usize); m]
    };
    let mut count = vec![HashSet::<usize>::new(); n+1];
    let mut graph = vec![vec![]; n+1];
    for &(a, b) in &abm {
        graph[a].push(b);
        graph[b].push(a);
    }
    for i in 1..=n {
        for j in 0..graph[i].len() {
            for k in j+1..graph[i].len() {
                if graph[graph[i][j]].contains(&graph[i][k]) {
                    continue;
                }
                count[graph[i][j]].insert(graph[i][k]);
                count[graph[i][k]].insert(graph[i][j]);
            }
        }
    }
    for i in 1..=n {
        println!("{}", count[i].len());
    }
}
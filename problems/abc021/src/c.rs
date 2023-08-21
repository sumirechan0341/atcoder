use std::collections::VecDeque;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        a: usize,
        b: usize,
        m: usize,
        xym: [(usize, usize); m]
    };
    let mut p = 1000000007;
    let mut graph = vec![vec![]; n+1];
    for &(x, y) in &xym {
        graph[x].push(y);
        graph[y].push(x);
    }
    let mut queue = VecDeque::<usize>::new();
    queue.push_back(a);
    let mut used = vec![-1; n+1];
    let mut way = vec![0; n+1];
    way[a] = 1;
    used[a] = 1;
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        for &next in &graph[current] {
            if used[next] == -1 {
                used[next] = used[current] + 1;
                queue.push_back(next);
            }
            if used[next] == used[current]+1 {
                way[next] += way[current];
                way[next] %= p;
            }
        }
    }
    println!("{}", way[b]);
}
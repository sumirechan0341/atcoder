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
    let mut used = vec![false; n];
    let mut q = VecDeque::<usize>::new();
    q.push_back(y-1);
    let mut path = VecDeque::<Vec<String>>::new();
    let mut ans = vec![];
    path.push_back(vec![]);
    while !q.is_empty() {
        let current = q.pop_front().unwrap();
        let p = &mut path.pop_front().unwrap();
        // let mut local_path = vec![];
        p.push((current+1).to_string());
        used[current] = true;
        if current == x-1 {
            ans = p.to_vec();
            break;
        }
        for next in &graph[current] {
            if !used[*next] {
                path.push_front(p.to_vec());
                q.push_front(*next);
                used[*next] = true;
            }
        }
    }
    ans.reverse();
    println!("{}", ans.join(" "));
}
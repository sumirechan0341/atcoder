use std::collections::{VecDeque, HashSet};

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        mut abm: [(usize, usize); m]
    };
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        graph[abm[i].0-1].push(abm[i].1-1);
    }
    let mut count = HashSet::<(usize, usize)>::new();
    for i in 0..n {
        let mut used = vec![false; n];
        let mut q = VecDeque::<usize>::new();
        q.push_back(i);
        while !q.is_empty() {
            let current = q.pop_front().unwrap();
            if used[current] {
                continue;
            }
            used[current] = true;
            count.insert((i, current));
            for j in &graph[current] {
                if used[*j] {
                    continue;
                }
                q.push_front(*j);
            }
        }
    }
    println!("{}", count.len());
}
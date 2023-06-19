use std::collections::VecDeque;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        uvm: [(usize, usize); m]
    };
    let mut graph = vec![vec![]; n];
    for uv in uvm {
        graph[uv.0-1].push(uv.1-1);
        graph[uv.1-1].push(uv.0-1);
    }
    let mut searched = vec![false; n];
    let mut s = 0;
    for i in 0..n {
        if searched[i] {
            continue;
        }
        s += 1;
        let mut q = VecDeque::<usize>::new();
        q.push_back(i);

        while !q.is_empty() {
            let from = q.pop_front().unwrap();
            searched[from] = true;
            for to in &graph[from] {
                if !searched[*to] {
                    searched[*to] = true;
                    q.push_back(*to);
                }
            }
        }
    }
    println!("{}", s);
}
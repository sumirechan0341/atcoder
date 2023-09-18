use std::collections::{HashMap, VecDeque};

use proconio::{input, marker::Chars, input_interactive};
pub fn main() {
    input! {
        n: usize,
        m: usize,
        uvm: [(usize, usize); m]
    };
    let mut graph = vec![vec![]; n+1];
    let mut edge_nums = vec![0; n+1];
    for &(u, v) in &uvm {
            graph[u].push(v);
            graph[v].push(u);
            edge_nums[u] += 1;
            edge_nums[v] += 1;
    }
    let mut used = vec![false; n+1];
    for i in 1..=n {
        if used[i] {
            continue;
        }
        let mut edge_num = 0;
        let mut vertex_num = 1;
        let mut queue = VecDeque::<usize>::new();
        queue.push_back(i);
        used[i] = true;
        while let Some(current) = queue.pop_front() {
            edge_num += edge_nums[current];
            for &next in &graph[current] {
                if !used[next] {
                    vertex_num += 1;
                    queue.push_back(next);
                    used[next] = true;
                }
            }
        }
        if edge_num/2 != vertex_num {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}
use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        abm: [(usize, usize); m]
    };
    let mut graph = vec![HashSet::<usize>::new(); n+1];
    for (a, b) in abm {
        graph[a].insert(b);
        graph[b].insert(a);
    }
    let next = &graph[1];
    for nt in next {
        if graph[*nt].contains(&n) {
            println!("{}", "POSSIBLE");
            return;
        }
    }
    println!("{}", "IMPOSSIBLE");
}
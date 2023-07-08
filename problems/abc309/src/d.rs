use std::collections::VecDeque;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n1: usize,
        n2: usize,
        m: usize,
        abm: [(usize, usize); m]
    };
    let mut graph = vec![vec![]; n1+n2+1];
    for ab in abm {
        graph[ab.0].push(ab.1);
        graph[ab.1].push(ab.0);
    }

    let mut queue = VecDeque::<(usize, usize)>::new();
    let mut used = vec![false; n1+n2+1];
    let mut path_len = vec![200000; n1+n2+1];
    // グラフ ある点からの距離を持ちたいパターン
    // パス path length BFS distance 経路長
    queue.push_back((1, 0));
    path_len[1] = 0;
    path_len[n1+n2]=0;

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if path_len[current.0] > current.1 {

            path_len[current.0] = current.1;
        }
        if used[current.0] {
            continue;
        }
        used[current.0] = true;
        
        for next in &graph[current.0] {
            if path_len[*next] > current.1+1 {
                path_len[*next] = current.1+1;
            }
            if used[*next] {
                continue;
            } else {
                queue.push_back((*next, current.1+1));
            }
        }
    }

    queue.push_back((n1+n2, 0));

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if path_len[current.0] > current.1 {
            path_len[current.0] = current.1;
        }
        if used[current.0] {
            continue;
        }
        used[current.0] = true;
        
        for next in &graph[current.0] {
            if path_len[*next] > current.1+1 {
                path_len[*next] = current.1+1;
            }
            queue.push_back((*next, current.1+1));
        }
    }
    println!("{}", path_len[1..=n1].into_iter().max().unwrap()+path_len[n1+1..=n1+n2].into_iter().max().unwrap()+1);
}
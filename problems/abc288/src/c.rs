use std::{collections::{HashSet, VecDeque}, vec};

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        abm: [(usize, usize); m]
    };
    // let mut adj = vec![vec![false; n]; n];
    let mut graph = vec![vec![]; n];
    for ab in abm {
        graph[ab.0-1].push(ab.1-1);
        graph[ab.1-1].push(ab.0-1);
    }
    
    // 連結でない部分同士で閉路はもたないので連結成分内のことのみ考える
    // 連結成分の頂点数がNi個で辺の数がMi個のとき、
    // 最大の辺の数はNi-1でそこから辺が増える分だけ閉路ができる
    // その余分な辺をすべての連結成分を足し合わせて
    //  S(Mi - (Ni - 1)) { i = 0..連結成分の数 }
    // M - N + 連結成分の数
    let mut searched = vec![false; n];
    let mut s = 0;
    for i in 0..n {
        if searched[i] {
            continue;
        } else {
            s += 1;
            let mut q = VecDeque::<usize>::new();
            q.push_back(i);
            while !q.is_empty() {
                let current = q.pop_front().unwrap();
                searched[current] = true;
                for j in &graph[current] {
                    if !searched[*j] {
                        q.push_back(*j);
                    }
                }
            }
        }
    }
    println!("{}", m+s-n);
}
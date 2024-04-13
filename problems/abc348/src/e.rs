use std::collections::HashMap;

use num::range;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        abn1: [(usize, usize); n-1],
        cn: [usize; n]
    };
    let mut cnt = vec![0; n];
    let mut graph = vec![vec![]; n];
    for i in 0..n - 1 {
        cnt[abn1[i].0 - 1] += 1;
        cnt[abn1[i].1 - 1] += 1;
        graph[abn1[i].0 - 1].push(abn1[i].1 - 1);
        graph[abn1[i].1 - 1].push(abn1[i].0 - 1);
    }
    let mut start = n + 1;
    let mut end = n + 1;
    for i in 0..n {
        if cnt[i] == 1 && start == n + 1 {
            start = i;
        } else if cnt[i] == 1 {
            end = i;
        }
    }
    if n == 1 {
        println!("{}", 0);
        return;
    }
    if n == 2 {
        println!("{}", cn[0].min(cn[1]));
        return;
    }
    let mut ls = vec![0; n + 1];
    let mut now = start;
    ls[1] = cn[start];

    let mut visited = vec![false; n];
    visited[start] = true;
    let mut i = 0;
    println!("{:?}", start);
    loop {
        i += 1;
        let nnow = graph[now].iter().find(|&&x| !visited[x]);
        println!("{:?}", nnow);
        if nnow == None {
            break;
        } else {
            now = *nnow.unwrap();
        }
        visited[now] = true;
        ls[i + 1] = ls[i] + cn[now];
    }

    let mut rs = vec![0; n + 1];
    let mut now = end;
    rs[1] = cn[end];
    let mut visited = vec![false; n];
    visited[end] = true;
    let mut i = 0;
    loop {
        i += 1;
        let nnow = graph[now].iter().find(|&&x| !visited[x]);
        if nnow == None {
            break;
        } else {
            now = *nnow.unwrap();
        }
        visited[now] = true;
        rs[i + 1] = rs[i] + cn[now];
    }
    let mut ans = usize::MAX;
    println!("{:?}", ls);
    println!("{:?}", rs);
    for i in 0..n {
        if ans > ls[i] + rs[n - i] {
            ans = ls[i] + rs[n - i];
        }
    }
    println!("{}", ans);
}

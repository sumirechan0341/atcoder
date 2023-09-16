use std::collections::VecDeque;

use proconio::{input, marker::Chars, input_interactive};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        abxym: [(usize, usize, i64, i64); m]
    };
    let mut graph = vec![vec![]; n+1];
    for &(a, b, x, y) in &abxym {
        graph[a].push((b, (x, y)));
        graph[b].push((a, (-x, -y)));
    }
    let mut queue = VecDeque::<(usize, (i64, i64))>::new();
    queue.push_back((1, (0, 0)));
    let mut used = vec![false; n+1];
    let mut ans = vec![(0, 0); n+1];
    while let Some((current, (x, y))) = queue.pop_back() {
        if used[current] {
            continue;
        }
        used[current] = true;
        for &(next, (nx, ny)) in &graph[current] {
            if used[next] {
                continue;
            }
            ans[next] = (ans[current].0 + nx, ans[current].1 + ny);
            queue.push_back((next, (ans[current].0 + nx, ans[current].1 + ny)));
        }
    }
    for i in 1..=n {
        if used[i] {
            println!("{} {}", ans[i].0, ans[i].1);
        } else {
            println!("{}", "undecidable");
        }
    }
}
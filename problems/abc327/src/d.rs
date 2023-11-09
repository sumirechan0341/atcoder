use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        am: [usize; m],
        bm: [usize; m]
    };
    let mut graph = vec![vec![]; n+1];
    for i in 0..m {
        graph[am[i]].push(bm[i]);
        graph[bm[i]].push(am[i]);
    }
    let mut cost = vec![-1; n+1];
    for i in 1..=n {
        let mut queue = VecDeque::new();
        if cost[i] != -1 {
            continue;
        }
        cost[i] = 0;
        queue.push_back((i, 0));
        while let Some((current, now_cost)) = queue.pop_front() {
            for &next in &graph[current] {
                if cost[next] != -1 {
                    if cost[next]%2 == now_cost%2 {
                        println!("{}", "No");
                        return;
                    }
                    continue;
                }
                cost[next] = now_cost+1;
                queue.push_back((next, now_cost+1));
            }
        }
    }
    println!("{}", "Yes");
}
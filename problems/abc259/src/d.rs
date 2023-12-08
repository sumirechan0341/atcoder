use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        sx: i64,
        sy: i64,
        tx: i64,
        ty: i64,
        xyrn: [(i64, i64, i64); n]
    };
    let mut queue = VecDeque::new();
    let mut goal = HashSet::new();
    for i in 0..n {
        if (sx - xyrn[i].0).pow(2) + (sy - xyrn[i].1).pow(2) == xyrn[i].2.pow(2) {
            queue.push_back(i);
        }
        if (tx - xyrn[i].0).pow(2) + (ty - xyrn[i].1).pow(2) == xyrn[i].2.pow(2) {
            goal.insert(i);
        }
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if (xyrn[i].0 - xyrn[j].0).pow(2) + (xyrn[i].1 - xyrn[j].1).pow(2)
                <= (xyrn[i].2 + xyrn[j].2).pow(2)
                && (xyrn[i].0 - xyrn[j].0).pow(2) + (xyrn[i].1 - xyrn[j].1).pow(2)
                    >= (xyrn[i].2 - xyrn[j].2).pow(2)
            {
                graph[i].push(j);
            }
        }
    }
    let mut used = vec![false; n];
    while let Some(c) = queue.pop_back() {
        if goal.contains(&c) {
            println!("{}", "Yes");
            return;
        }
        used[c] = true;
        for &next in &graph[c] {
            if used[next] {
                continue;
            }
            queue.push_back(next);
        }
    }
    println!("{}", "No");
}

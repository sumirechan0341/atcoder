use std::collections::{HashMap, HashSet, VecDeque};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i64; n],
        pn1: [usize; n-1]
    };
    let mut aan = an.clone();
    let mut queue = VecDeque::new();
    let mut graph = vec![vec![]; n + 1];
    for i in 0..n - 1 {
        graph[pn1[i]].push(i + 2);
    }
    let mut used = vec![false; n + 2];
    let mut connected = HashMap::new();

    for &x in &graph[1] {
        queue.push_back((x, 1));
        connected.insert(x, 1);
    }
    connected.insert(1, 0usize);
    while let Some((x, depth)) = queue.pop_front() {
        used[x] = true;
        for &y in &graph[x] {
            if !used[y] {
                queue.push_back((y, depth + 1));
            }
            connected
                .entry(y)
                .and_modify(|x| *x = depth + 1)
                .or_insert(depth + 1);
        }
    }
    let mut ans = vec![0; 2500001];
    for i in 0..n - 1 {
        aan[pn1[i] - 1] = aan[pn1[i] - 1] + aan[i + 1];
        if connected.contains_key(&(i + 2)) {
            ans[connected[&(i + 2)]] += aan[i + 1];
        }
    }
    for i in (0..2500001).rev() {
        if ans[i] > 0 {
            println!("{}", "+");
            return;
        }
        if ans[i] < 0 {
            println!("{}", "-");
            return;
        }
    }
    println!(
        "{}",
        if an[0] > 0 {
            "+"
        } else if an[0] < 0 {
            "-"
        } else {
            "0"
        }
    );
}

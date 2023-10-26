use std::collections::{HashSet, VecDeque, HashMap};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        stn: [(String, String); n]
    };
    let mut used = vec![false; n];
    let mut start_map = HashMap::new();
    let mut dist_map = HashMap::new();
    let mut go = vec![vec![]; n];
    let mut back = vec![vec![]; n];
    for i in 0..n {
        start_map.insert(&stn[i].0, i);
        dist_map.insert(&stn[i].1, i);
    }
    for i in 0..n {
        if let Some(&x) = start_map.get(&(stn[i].1)) {
            go[i].push(x);
        }
        if let Some(&x) = dist_map.get(&(stn[i].0)) {
            back[i].push(x);
        }
    }
    for i in 0..n {
        if used[i] {
            continue;
        }
        used[i] = true;
        let mut queue = VecDeque::new();
        queue.push_back(i);
        while let Some(current) = queue.pop_back() {
            used[current] = true;
            for &next in &go[current] {
                if used[next] {
                    println!("{}", "No");
                    return;
                }
                queue.push_back(next);
            }
        }
        queue.push_back(i);
        while let Some(current) = queue.pop_back() {
            used[current] = true;
            for &next in &back[current] {
                if used[next] {
                    println!("{}", "No");
                    return;
                }
                queue.push_back(next);
            }
        }
    }
    println!("{}", "Yes");
}
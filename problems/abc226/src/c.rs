use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
    };
    let mut waza = vec![];
    for _ in 0..n {
        input! {
            t: i128,
            k: usize,
            ak: [usize; k]
        };
        waza.push((t, ak));
    }
    let mut queue = VecDeque::<usize>::new();
    let mut used = vec![false; n+1];
    let mut time = waza[n-1].0;
    queue.push_back(n);
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        for next in &waza[current-1].1 {
            if !used[*next] {
                used[*next] = true;
                queue.push_front(*next);
                time += waza[*next-1].0;
            }
        }
    }
    println!("{}", time);
}
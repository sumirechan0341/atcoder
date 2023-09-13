use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n],
        m: usize,
        bm: [usize; m],
        x: usize
    };
    let mut mochi = vec![false; x+1];
    for &b in &bm {
        mochi[b] = true;
    }
    let mut queue = VecDeque::<usize>::new();
    let mut vis = vec![false; x+1];
    queue.push_back(x);
    vis[x] = true;
    while let Some(next) = queue.pop_front() {
        for &a in &an {
            if next < a || mochi[next-a] || vis[next-a] {
                continue;
            }
            vis[next-a] = true;
            queue.push_back(next-a);
        }
    }
    println!("{}", if vis[0] { "Yes" } else { "No" });
}
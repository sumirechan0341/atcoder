use std::collections::{HashSet, BTreeSet};

use num::traits::Pow;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        k: usize
    };
    let ans = &mut BTreeSet::<usize>::new();
    dfs(ans, 10, 0);
    println!("{}", ans.iter().nth(k).unwrap());
}

fn dfs(ans: &mut BTreeSet<usize>, rd: usize, now: usize) {
    ans.insert(now);
    if rd == 0 {
        // pass
    }
    for i in 0..rd {
        dfs(ans, i, now*10+i);
    }
}
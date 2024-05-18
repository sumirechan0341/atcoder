use std::collections::{BinaryHeap, HashSet};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        acn: [(usize, usize); n]
    };
    let mut iacn = acn.iter().enumerate().collect::<Vec<_>>();
    iacn.sort_by_key(|x| x.1);
    let mut ans = vec![];
    let mut used = HashSet::new();
    for i in 0..n {
        if used.contains(&iacn[i].1 .1) {
            continue;
        } else {
            used.insert(iacn[i].1 .1);
            ans.push(iacn[i]);
        }
    }
    ans.sort_by_key(|x| x.1 .1);
    let mut an = ans.iter().copied().map(|x| x.1 .0).collect::<Vec<_>>();
    an.sort();
    let mut without = HashSet::new();
    let mut ans2 = ans.clone();
    ans2.sort_by_key(|x| x.1 .0);
    let mut now = 0;
    for i in 0..ans.len() {
        // println!("without {:?}", without);
        // println!("{:?}", ans[i]);
        if without.contains(&ans[i]) {
            continue;
        }
        let a = ans[i].1 .0;
        let idx = an.partition_point(|x| x < &a);
        // println!("(idx, i){:?}", (idx, i));
        if idx < i {
            without.insert(ans[i]);
            continue;
        }
        ans2[now..idx].iter().for_each(|x| {
            without.insert(*x);
        });
        now = idx + 1;
    }
    ans.sort_by_key(|x| x.0);
    println!("{}", ans.len() - without.len());
    for i in 0..ans.len() {
        if without.contains(&ans[i]) {
            continue;
        } else {
            println!("{}", ans[i].0 + 1);
        }
    }
}

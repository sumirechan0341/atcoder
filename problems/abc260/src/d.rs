use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        pn: [usize; n]
    };

    let mut eaten = vec![-1; n + 1];
    let mut linked = vec![0; n + 1];
    let mut stacks = BTreeSet::new();
    for i in 0..n {
        let upper = stacks.range(&(pn[i], 0)..);
        if let Some(&(min, m)) = upper.min() {
            linked[pn[i]] = min;
            if m + 1 == k {
                eaten[pn[i]] = i as i32 + 1;
                let mut next = linked[pn[i]];
                while next != 0 {
                    eaten[next] = i as i32 + 1;
                    next = linked[next];
                }
            } else {
                stacks.insert((pn[i], m + 1));
            }
            stacks.remove(&(min, m));
        } else {
            stacks.insert((pn[i], 1));
        }
    }
    if k == 1 {
        for i in 0..n {
            eaten[pn[i]] = i as i32 + 1;
        }
    }
    for i in 1..=n {
        println!("{}", eaten[i]);
    }
}

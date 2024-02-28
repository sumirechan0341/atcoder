use std::collections::{BTreeSet, HashMap, HashSet};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        qq: [(usize, usize, usize); q]
    };
    let mut set = BTreeSet::new();
    for i in 0..n - 1 {
        if s[i] == s[i + 1] {
            set.insert(i + 1);
        }
    }
    for (id, l, r) in qq {
        if id == 1 {
            if set.contains(&(l - 1)) {
                set.remove(&(l - 1));
            } else {
                if l != 1 {
                    set.insert(l - 1);
                }
            }
            if set.contains(&r) {
                set.remove(&r);
            } else {
                if r != n {
                    set.insert(r);
                }
            }
        } else {
            let a = set.range(..l).next_back().unwrap_or(&0);
            let b = set.range(..r).next_back().unwrap_or(&0);
            if a == b {
                println!("{}", "Yes");
            } else {
                println!("{}", "No");
            }
        }
    }
}

use std::collections::{HashSet, BTreeSet};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        sh: [Chars; h],
        th: [Chars; h]
    };
    // multiset
    let mut shset = BTreeSet::<(Vec<char>, usize)>::new();
    for i in 0..w {
        let mut v = vec![];
        for j in 0..h {
            v.push(sh[j][i]);
        }
        shset.insert((v, i));
    }
    for i in 0..w {
        let mut v = vec![];
        for j in 0..h {
            v.push(th[j][i]);
        }        

        // この記法便利
        // (v, 0)が下限
        // (v, w)が上限
        if let Some((v1, i)) = shset.range((v.clone(), 0)..=(v, w)).nth(0) {
            shset.remove(&(v1.to_vec(), *i));
        } else {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}
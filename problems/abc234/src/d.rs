use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        pn: [usize; n]
    };
    let mut used = vec![false; n + 1];
    for i in 0..k {
        used[pn[i]] = true;
    }
    let mut current = 0;
    while !used[current] {
        current += 1;
    }
    println!("{}", current);
    for i in k..n {
        used[pn[i]] = true;
        if current > pn[i] {
            println!("{}", current);
        } else {
            current += 1;
            while !used[current] {
                current += 1;
            }
            println!("{}", current);
        }
    }
    let mut set = BTreeSet::new();
    for i in 0..k {
        set.insert(pn[i]);
    }
    println!("{}", set.first().unwrap());
    for i in k..n {
        if set.first().unwrap() < &pn[i] {
            set.insert(pn[i]);
            set.pop_first();
        }
        println!("{}", set.first().unwrap());
    }
}

use std::collections::{HashMap, BTreeMap, HashSet};
use std::iter::FromIterator;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n]
    };
    let mut anset = HashSet::<usize>::from_iter(an.clone());
    let anset2 = HashSet::<usize>::from_iter(an.clone());
    let mut anset_vec = anset2.iter().collect::<Vec<_>>();
    anset_vec.sort();
    let mut remain = an.len()-anset.len();
    for i in 1.. {
        if !anset.contains(&i) {
            if remain / 2 > 0 {
                remain -= 2;
                continue;
            } 
            if let Some(x) = anset_vec.pop() {
                if remain == 1 {
                    remain = 0;
                    if !anset.remove(&x) {
                        println!("{}", i-1);
                        return;
                    }
                } else {
                    if let Some(x2) = anset_vec.pop() {
                        if !anset.remove(&x) || !anset.remove(&x2) {
                            println!("{}", i-1);
                            return;
                        }
                    } else {
                        println!("{}", i-1);
                        return;
                    }
                }
            } else {
                println!("{}", i-1);
                return;
            }
        } else {
            anset.remove(&i);
        }
    }
}
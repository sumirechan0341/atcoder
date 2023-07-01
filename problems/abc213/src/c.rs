use std::collections::{BTreeSet, HashSet};
use std::iter::FromIterator;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abn: [(usize, usize); n]
    };
    let new_w = HashSet::<usize>::from_iter(abn.iter().map(|x| x.1));
    let new_h = HashSet::<usize>::from_iter(abn.iter().map(|x| x.0));
    let mut sorted_w = new_w.clone().into_iter().collect::<Vec<_>>();
    sorted_w.sort();
    let mut sorted_h = new_h.clone().into_iter().collect::<Vec<_>>();
    sorted_h.sort();
    for i in 0..n {
        let y = match sorted_h.binary_search(&abn[i].0) {
            Ok(a) => {
                a
            },
            Err(a) => {
                a
            }
        };
        let x = match sorted_w.binary_search(&abn[i].1) {
            Ok(a) => {
                a
            },
            Err(a) => {
                a
            }
        };
        println!("{} {}", y+1, x+1);
    }
}
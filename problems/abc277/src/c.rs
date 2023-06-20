use std::collections::{VecDeque, HashSet, HashMap};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        abn: [(usize, usize); n]
    };
    let mut q = VecDeque::<usize>::new();
    let mut m = HashMap::<usize, Vec<usize>>::new();
    let mut used = vec![false; 1000000000];
    for ab in abn {
        match m.get_mut(&ab.0) {
            Some(x) => {
                x.push(ab.1)
            },
            None => {
                m.insert(ab.0, vec![ab.1]);
            }
        }
        match m.get_mut(&ab.1) {
            Some(x) => {
                x.push(ab.0)
            },
            None => {
                m.insert(ab.1, vec![ab.0]);
            }
        }
    }
    q.push_back(1);
    let mut max = 1;
    while !q.is_empty() {
        let current = q.pop_front().unwrap();
        if current > max {
            max = current;
        }
        used[current-1] = true;
        match m.get(&current) {
            Some(x) => {
                for e in x {
                    if !used[e-1] {
                        used[e-1] = true;
                        q.push_back(*e);
                    }
                }
            },
            None => {

            }
        }
    }
    println!("{}", max);
}
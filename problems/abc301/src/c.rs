use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars,
        t: Chars
    };
    let mut sset = HashMap::<char, i32>::new();
    let mut tset = HashMap::<char, i32>::new();
    for c in s {
        match sset.get_mut(&c) {
            Some(x) => {
                *x += 1;
            },
            None => {
                sset.insert(c, 1);
            }
        } 
    }
    for c in t {
        match tset.get_mut(&c) {
            Some(x) => {
                *x += 1;
            },
            None => {
                tset.insert(c, 1);
            }
        } 
    }
    let mut aks = HashSet::<&char>::new();
    for c in sset.keys() {
        aks.insert(c);
    }
    for c in tset.keys() {
        aks.insert(c);
    }
    let mut cost = 0;
    for key in aks {
        if key == &'@' {
            cost += sset.get(key).unwrap_or(&0) + tset.get(key).unwrap_or(&0);
            continue;
        }
        match key {
            'a' | 't' | 'c' | 'o' | 'd' | 'e' | 'r' => {
                cost -= (tset.get(key).unwrap_or(&0) - sset.get(key).unwrap_or(&0)).abs()
            },
            _ => {
                if tset.get(key) != sset.get(key) {
                    println!("{}", "No");
                    return;
                }
            }
        }
    }
    println!("{}", if cost >= 0 { "Yes" } else { "No" });
}
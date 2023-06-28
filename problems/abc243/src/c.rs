use std::collections::{HashSet, HashMap, BTreeMap, BTreeSet};

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        xyn: [(i32, i32); n],
        s: Chars
    };
    // y,x
    let mut lmap = BTreeSet::<(i32, i32)>::new();
    let mut rmap = BTreeSet::<(i32, i32)>::new();
    for i in 0..n {
        match s[i] {
            'R' => {
                if lmap.range((xyn[i].1, xyn[i].0)..(xyn[i].1, std::i32::MAX)).count() > 0 {
                    println!("{}", "Yes");
                    return;
                }
                rmap.insert((xyn[i].1, xyn[i].0));
            },
            _ => {
                if rmap.range((xyn[i].1, std::i32::MIN)..(xyn[i].1, xyn[i].0)).count() > 0 {
                    println!("{}", "Yes");
                    return;
                }
                lmap.insert((xyn[i].1, xyn[i].0));
            }
        }
        
    }
    println!("{}", "No");
}
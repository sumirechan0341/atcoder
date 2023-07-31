use std::collections::{HashSet, BTreeSet};
use std::iter::FromIterator;
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars,
        mut k: usize
    };
    let mut sc = BTreeSet::<char>::from_iter(s.clone());
    let mut ans_set = BTreeSet::<&[char]>::new();
    for i in 0..5 {
        let current = sc.iter().nth(i).unwrap();
        for j in 0..s.len() {
            if s[j] == *current {
                for l in 0..(s.len()-j).min(5) {
                    ans_set.insert(&s[j..=j+l]);
                }
            }
        }
        if ans_set.len() >= k {
            break;
        }
    }
    println!("{}", ans_set.iter().nth(k-1).unwrap().iter().collect::<String>());
}
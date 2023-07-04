use std::collections::{HashSet, HashMap};

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i64; n]
    };
    let mut map = HashMap::<i64, i64>::new();
    for a in &an {
        map.entry(*a).and_modify(|x| *x+=1).or_insert(1);
    }
    println!("{}", ((an.len()*(an.len()-1))/2) as i64 - map.values().map(|x| (x*(x-1))/2).sum::<i64>());
}
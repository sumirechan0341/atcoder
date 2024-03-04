use std::collections::{HashMap, HashSet};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        t: usize,
        mut abt: [(usize, usize); t]
    };
    let mut map = HashMap::new();
    map.insert(0, n);
    let mut score = vec![0; n];
    for i in 0..t {
        let x = map.get_mut(&score[abt[i].0 - 1]).unwrap();
        if *x == 1 {
            map.remove(&score[abt[i].0 - 1]);
        } else {
            *x -= 1;
        }
        score[abt[i].0 - 1] += abt[i].1;
        if map.contains_key(&score[abt[i].0 - 1]) {
            map.entry(score[abt[i].0 - 1]).and_modify(|x| *x += 1);
        } else {
            map.insert(score[abt[i].0 - 1], 1);
        }
        println!("{}", map.len());
    }
}

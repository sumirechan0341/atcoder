use std::collections::HashMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        acn: [(usize, usize); n]
    };
    let mut mins = HashMap::new();
    for i in 0..n {
        let a = acn[i].0;
        let c = acn[i].1;
        if let None = mins.get(&c) {
            mins.insert(c, a);
        } else if mins.get(&c) > Some(&a) {
            mins.insert(c, a);
        }
    }
    println!("{}", mins.iter().sorted_by_key(|x| x.1).last().unwrap().1);
}

use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        stn: [(String, i32); n]
    };
    let mut used = HashSet::<&String>::new();
    let mut max_i = 0;
    for i in 0..n {
        if !used.contains(&stn[i].0) {
            used.insert(&stn[i].0);
            if stn[max_i].1 < stn[i].1 {
                max_i = i;
            }
        }
    }
    println!("{}", max_i+1);
}
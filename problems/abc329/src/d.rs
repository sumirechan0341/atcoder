use std::collections::BTreeMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        am: [usize; m]
    };
    let mut count = vec![0; n+1];
    let mut index = 0;
    let mut max = 0;
    for i in 0..m {
        count[am[i]] += 1;
        if count[am[i]] > max {
            max = count[am[i]];
            index = am[i];
        } else if count[am[i]] == max && am[i] < index {
            index = am[i];
        }
        println!("{}", index);
    }
}
use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        an: [usize; n]
    };
    let mut aset = HashSet::new();
    for i in 0..n {
        if an[i] <= k {
            aset.insert(an[i]);
        }
    }
    println!("{}", k * (k + 1) / 2 - aset.iter().sum::<usize>());
}

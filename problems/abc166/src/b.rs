use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut safe = HashSet::<usize>::new();
    for i in 0..k {
        input! {
            d: usize,
            ad: [usize ; d]
        };
        for a in ad {
            safe.insert(a);
        }
    }
    println!("{}", n-safe.len());
}
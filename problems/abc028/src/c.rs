use std::collections::{HashSet, BTreeSet};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        v5: [i32; 5]
    };
    let mut set = BTreeSet::<i32>::new();
    for i in 0..5 {
        for j in i+1..5 {
            for k in j+1..5 {
                set.insert(v5[i]+v5[j]+v5[k]);
            }
        }
    }
    println!("{}", set.iter().nth_back(2).unwrap());
}
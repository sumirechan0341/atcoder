use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        sn: [Chars; n]
    };
    let mut sset = HashSet::<Vec<char>>::new();
    for s in sn {
        if !"HDSC".to_string().contains(s[0]) {
            println!("{}", "No");
            return;
        }
        if !"A23456789TJQK".to_string().contains(s[1]) {
            println!("{}", "No");
            return;
        }
        sset.insert(s);
    }
    if sset.len() != n {
        println!("{}", "No");
        return;
    }
    println!("{}", "Yes");
}
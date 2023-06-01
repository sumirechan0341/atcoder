use std::collections::HashSet;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };
    let mut called: HashSet<usize> = HashSet::new();
    for i in 0..n {
        if !called.contains(&(i+1)) {
            called.insert(a[i]);
        }
    }
    println!("{}", n - called.len());
    let mut first = true;
    for i in 0..n {
        if !called.contains(&(i+1)) {
            if first {
                print!("{}", i + 1);
                first = false;
            } else {
                print!(" {}", i + 1);
            }
        }
    }
    println!("{}", "");
}
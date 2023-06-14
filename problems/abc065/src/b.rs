use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n]
    };
    let mut current: usize = 1;
    let mut searched = HashSet::<usize>::new();
    for i in 0.. {
        if current == 2 {
            println!("{}", i);
            return;
        }
        current = an[current-1];
        if searched.contains(&current) {
            break;
        }
        searched.insert(current);
    }
    println!("{}", -1);
}
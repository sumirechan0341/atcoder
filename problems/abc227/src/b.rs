use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        sn: [i32; n]
    };
    let mut ans = HashSet::<i32>::new();
    for i in 1..200 {
        for j in 1..200 {
            ans.insert(4*i*j + 3*i + 3*j);
        }
    }
    let mut count = 0;
    for i in 0..n {
        if !ans.contains(&sn[i]) {
            count += 1;
        }
    }
    println!("{}", count);
}
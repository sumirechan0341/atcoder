use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut list = HashMap::<usize, Vec<i32>>::new();
    for i in 0..n {
        input! {
            l: usize,
            al: [i32; l]
        };
        list.insert(i+1, al);
    }
    input! {
        queries: [[usize; 2] ;q]
    };
    for st in queries {
        println!("{}", list.get(&st[0]).unwrap()[st[1]-1]);
    }
}
use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n]
    };
    let mut ftree = HashMap::<usize, i32>::new();
    ftree.insert(1, 0);

    for i in 1..=an.len() {
        match &ftree.get(&an[i-1]) {
            Some(&x) => {
                ftree.insert(2*i, x+1);
                ftree.insert(2*i+1, x+1);
            },
            _ => {

            }
        } 
    }
    for i in 1..=2*n+1 {
        println!("{}", ftree.get(&i).unwrap());
    }
}
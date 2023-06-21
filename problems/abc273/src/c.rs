use std::collections::{HashSet, HashMap};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut an: [usize; n]
    };
    let mut anmap = HashMap::<usize, i32>::new();
    for a in an {
        match anmap.get_mut(&a) {
            Some(x) => {
                *x += 1;
            },
            None => {
                anmap.insert(a, 1);
            }
        } 
    }
    let mut kvec = anmap.keys().collect::<Vec<_>>();
    kvec.sort();
    kvec.reverse();
    for i in 0..n {
        if i >= kvec.len() {
            println!("{}", 0);
        } else {
            println!("{}", anmap.get(kvec[i]).unwrap_or(&0));
        }
    }
}
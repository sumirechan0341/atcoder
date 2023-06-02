use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i32; n],
        q: usize
    };
    let mut nmap = HashMap::<usize, i32>::new();
    let mut ans: Vec<i32> = vec![];
    for (index, a) in an.iter().enumerate() {
        nmap.insert(index + 1, *a);
    }
    for i in 0..q {
        input! {
            qtype: usize
        }
        if qtype == 1 {
            input! {
                k: usize,
                x: usize
            };
            nmap.insert(k, x as i32);
        } else {
            input! {
                k: usize,
            };
            ans.push(*nmap.get(&k).unwrap());
        }
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n"));
}
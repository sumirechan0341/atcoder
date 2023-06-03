use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        mut t: i64,
        a: [i64; n-1],
        xym: [[i64; 2]; m]
    };
    let mut bonus = HashMap::<usize, i64>::new();
    for xy in xym {
        bonus.insert(xy[0] as usize, xy[1]);
    }
    for i in 0..n-1 {
        match bonus.get(&(i+1)) {
            Some(bt) => {
                t += bt;
            },
            _ => {

            }
        };
        t -= a[i];
        if t <= 0 {
            println!("{}", "No");
            return;
        } 
        
    }
    println!("{}", "Yes");

}
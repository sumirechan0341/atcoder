use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        dn: [i64; n],
        m: usize,
        tm: [i64; m]
    };
    let mut map = HashMap::<i64, i64>::new();
    for i in 0..n {
        map.entry(dn[i]).and_modify(|x| *x += 1).or_insert(1);
    }
    for i in 0..m {
        match map.get_mut(&tm[i]) {
            Some(x) => {
                *x -= 1;
                if *x == 0 {
                    map.remove(&tm[i]);
                }
            },
            None => {
                println!("{}", "NO");
                return;
            }
        }
    }
    println!("{}", "YES");
}
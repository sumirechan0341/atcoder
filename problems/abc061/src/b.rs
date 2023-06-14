use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m]
    };
    let mut map = HashMap::<usize, i32>::new();
    for ab in abm {
        match map.get_mut(&ab.0) {
            Some(x) => { *x += 1 },
            None => { map.insert(ab.0, 1); }
        };
        match map.get_mut(&ab.1) {
            Some(x) => { *x += 1 },
            None => { map.insert(ab.1, 1); }
        };
    }
    for i in 1..=n {
        println!("{}", map.get(&i).unwrap_or(&0));
    }
}
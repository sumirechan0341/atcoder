use std::collections::HashMap;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        sn: [String; n],
        m: usize,
        tm: [String; m]
    };
    let mut b = HashMap::<String, i32>::new();
    let mut r = HashMap::<String, i32>::new();
    for s in sn {
        match b.get_mut(&s) {
            Some(x) => { *x += 1; },
            None => { b.insert(s, 1); }
        }
    }
    for t in tm {
        match r.get_mut(&t) {
            Some(x) => { *x += 1; },
            None => { r.insert(t, 1); }
        }
    }
    let mut ans = 0;
    for bs in b {
        let score = bs.1 - r.get(&bs.0).unwrap_or(&0);
        if score > ans {
            ans = score;
        }
    }
    println!("{}", ans);
}
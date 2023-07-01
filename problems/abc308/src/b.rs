use std::collections::HashMap;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        cn: [String; n],
        dm: [String; m],
        pm1: [i32; m+1]
    };
    let mut total = 0;
    let mut price = HashMap::<String, i32>::new();
    for i in 0..m {
        price.insert(dm[i].to_string(), pm1[i+1]);
    }
    for c in cn {
        if let Some(p) = price.get(&c) {
            total += p;
        } else {
            total += pm1[0];
        }
    }
    println!("{}", total);
}
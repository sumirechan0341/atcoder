use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        sn: [String; n]
    };
    let mut candidate = HashMap::<String, i32>::new();
    for s in sn {
        match candidate.get(&s) {
            Some(v) => {
                candidate.insert(s, *v + 1);
            },
            None => {
                candidate.insert(s, 1);
            }
        };
    }
    println!("{}", candidate.iter().fold((&("".to_string()), &0), |acc, x| if acc.1 < x.1 { x } else { acc }).0);
}
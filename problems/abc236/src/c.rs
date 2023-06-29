use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        sn: [String; n],
        tm: [String; m]
    };
    let mut station = HashSet::<String>::new();
    for t in tm {
        station.insert(t);
    }
    for s in sn {
        if station.contains(&s) {
            println!("{}", "Yes");
        } else {
            println!("{}", "No");
        }
    }
}
use std::collections::HashMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        sn: [String; n]
    };
    let mut ans = HashMap::<String, i32>::new();
    for s in sn {
        match ans.get_mut(&s) {
            Some(x) => {
                *x += 1;
            },
            None => {
                ans.insert(s, 1);
            }
        }
    }
    println!("{}", ans.into_iter().sorted_by_key(|a| a.1).last().unwrap().0);
}
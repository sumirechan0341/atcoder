use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        t: usize,
        c: [usize; n],
        r: [usize; n]
    };
    let mut card_map: HashMap<usize, (usize, usize)> = HashMap::new();
    for i in 0..n {
        match card_map.get(&c[i]) {
            Some((r_max, _)) => {
                if r[i] > *r_max {
                    card_map.insert(c[i], (r[i], i+1));
                }
            },
            None => {
                card_map.insert(c[i], (r[i], i+1));
            }
        }
    }
    match card_map.get(&t) {
        Some((_, player)) => {
            println!("{}", player);
        },
        None => {
            println!("{}", card_map.get(&c[0]).unwrap().1);
        }
    }
}
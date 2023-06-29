use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        q: usize,
        an: [usize; n],
        xkq: [(usize, usize); q]
    };
    let mut map = HashMap::<(usize, usize), usize>::new();
    let mut count_map = HashMap::<usize, usize>::new();
    for i in 0..n {
        if let Some(y) = count_map.get_mut(&an[i]) {
            *y += 1;
            map.insert((an[i], *y), i+1);
        } else {
            map.insert((an[i], 1), i+1);
            count_map.insert(an[i], 1);
        }
    }
    for xk in xkq {
        if let Some(y) = map.get(&xk) {
            println!("{}", y);
        } else {
            println!("{}", -1);
        }
    }
}
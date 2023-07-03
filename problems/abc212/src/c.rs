use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        an: [i64; n],
        mut bm: [i64; m]
    };
    bm.sort();
    let mut min = std::i64::MAX;
    for i in 0..n {
        match bm.binary_search(&an[i]) {
            Ok(x) => {
                println!("{}", 0);
                return;
            },
            Err(x) => {
                let local_min = if x == m {
                    (an[i]-bm[m-1]).abs()
                } else if x == 0 {
                    (an[i]-bm[0]).abs()
                } else {
                    (an[i]-bm[x]).abs().min((an[i]-bm[x-1]).abs())
                };
                if min > local_min {
                    min = local_min
                }
            }
        }
    }
    println!("{}", min);
}
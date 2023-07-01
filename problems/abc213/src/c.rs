use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mut abn: [(usize, usize); n]
    };
    abn.sort();
    let mut ans = vec![];
    let mut row = 0;
    let mut col = 1;
    let mut prev_row = 0;
    for i in 0..n {
        if prev_row != abn[i].0 {
            col = 1;
            row += 1;
            prev_row = abn[i].0;
        } else {
            col += 1;
        }
        ans.push((row, col));
    }
    for a in ans {
        println!("{} {}", a.0, a.1);
    }
}
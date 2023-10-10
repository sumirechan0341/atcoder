use std::collections::HashMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        sn: [Chars; n]
    };
    let mut count = vec![];
    for i in 0..n {
        count.push((i+1, 0));
    }
    for i in 0..n {
        for j in 0..n {
            if sn[i][j] == 'o' {
                count[i].1 += 1;
            }
        }
    }
    // 集計値をインデックス順に表示するやつ
    count.sort_by_key(|x| -x.1);
    for i in 0..n {
        println!("{}", count[i].0);
    }

}
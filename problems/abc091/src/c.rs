use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        mut abn: [(usize, usize); n],
        mut cdn: [(usize, usize); n]
    };
    // 解説AC
    // 赤い点はy座標大きい順
    // 青い点はx座標の小さい順
    abn.sort_by_key(|x| x.1);
    abn.reverse();
    cdn.sort();
    let mut used = vec![vec![false; 201]; 201];
    let mut count = 0;
    for (c, d) in cdn {
        for (a, b) in &abn {
            if c > *a && d > *b && !used[*a][*b] {
                count += 1;
                used[*a][*b] = true;
                break;
            }
        }
    }
    println!("{}", count);
}
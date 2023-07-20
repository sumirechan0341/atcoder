use std::collections::HashMap;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        sn: [Chars; n]
    };
    let mut map = HashMap::<Vec<char>, i64>::new();
    for s in sn {
        let mut scp = s.clone();
        scp.sort();
        map.entry(scp).and_modify(|x| *x += 1).or_insert(1);
    }
    println!("{}", map.values().map(|x| x * (x-1) / 2).sum::<i64>());
}
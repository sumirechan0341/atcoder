use std::collections::BTreeMap;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        mut k: usize,
        abn: [(usize, usize); n]
    };
    let mut map = BTreeMap::<usize, usize>::new();
    for (a, b) in abn {
        map.entry(a).and_modify(|x| *x+=b).or_insert(b);
    }
    for (key, v) in map {
        if v >= k {
            println!("{}", key);
            return;
        } else {
            k -= v;
        }
    }
}
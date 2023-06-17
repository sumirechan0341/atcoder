use std::collections::{BTreeMap, HashMap};

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        k: usize,
        q: usize,
        xyq: [(i32, i32); q]
    };
    let mut used = HashMap::<i32, i32>::new();
    used.insert(0, 0);
    // let mut max_keys = vec![];
    let mut ans: Vec<i32> = vec![];
    for xy in xyq {
        match used.get_mut(&xy.0) {
            Some(x) => {
                if *x < xy.1 {
                    *x = xy.1;
                }
            },
            None => {
                used.insert(xy.0, xy.1);
            }
        }
        
    }
    // for i in 0..q {
    //     println!("{}", ans[i]);
    // }
}
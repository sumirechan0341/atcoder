use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i32; n]
    };
    let mut map = HashMap::new();
    for i in 0..n {
        map.insert(an[i], i);
    }
    let mut index = -1;

    for i in 0..n {
        let ans = map.get(&(index)).unwrap();
        println!("{}", ans + 1);
        index = (*ans + 1) as i32;
    }
}

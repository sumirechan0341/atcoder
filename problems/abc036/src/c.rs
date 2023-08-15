use std::collections::HashMap;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i32; n]
    };
    let mut map = HashMap::<i32, i32>::new();
    let mut an_c = an.clone();
    an_c.sort();
    let mut index = 0;
    for a in an_c {
        if !map.contains_key(&a) {
            map.insert(a, index);
            index += 1;
        }
    }
    for a in an {
        println!("{}", map.get(&a).unwrap());
    }
}
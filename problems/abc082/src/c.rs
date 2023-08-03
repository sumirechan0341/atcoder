use std::collections::HashMap;
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i32; n]
    };
    let mut map = HashMap::<i32, i32>::new();
    for i in 0..n {
        map.entry(an[i]).and_modify(|x| *x+=1).or_insert(1);
    }
    let mut total = 0;
    for (k, v) in map {
        if k > v {
            total += v;
        }
        if k < v {
            total += v-k;
        }
    }
    println!("{}", total);
}
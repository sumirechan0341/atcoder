use std::collections::HashMap;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i64; n]
    };
    let mut map = HashMap::<i64, i64>::new();
    for i in 0..n {
        map.entry(an[i]).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut valids = map.iter().filter(|x| x.1 >= &2).collect::<Vec<_>>();
    valids.sort();
    valids.reverse();
    if valids.len() < 2 {
        println!("{}", 0);
        return;
    }
    if valids[0].1 >= &4 {
        println!("{}", valids[0].0*valids[0].0);
        return;
    }
    println!("{}", valids[0].0*valids[1].0);
}
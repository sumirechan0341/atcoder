use std::collections::HashMap;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        sn: [String; n]
    };
    let mut vote = HashMap::<String, i32>::new();
    for s in sn {
        vote.entry(s).and_modify(|x| *x += 1).or_insert(1);
    }
    let max = vote.values().max().unwrap();
    let mut ans = vote.iter().collect::<Vec<_>>();
    ans.sort();
    for (s, i) in ans {
        if i == max {
            println!("{}", s);
        }
    }
}
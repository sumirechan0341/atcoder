use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i32; n]
    };
    let mut ans = HashSet::<i32>::new();
    for i in 0..n {
        if ans.contains(&an[i]) {
            ans.remove(&an[i]);
        } else {
            ans.insert(an[i]);
        }
    }
    println!("{}", ans.len());
}
use std::collections::HashMap;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        an: [i64; n]
    };
    let mut ans = HashMap::<i64, i64>::new();
    for i in 0..n {
        match ans.get_mut(&an[i]) {
            Some(x) => {
                *x += 1;
            },
            None => {
                ans.insert(an[i], 1);
            }
        }
    }
    println!("{}", ans.values().map(|&x| x / &2).sum::<i64>());
}
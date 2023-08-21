use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i64; n]
    };
    let mut ans = HashSet::<i64>::new();
    for i in 0..n {
        let mut a = an[i];
        while a & 1 == 0 {
            a = a >> 1;
        }
        ans.insert(a);
    }
    println!("{}", ans.len());
}
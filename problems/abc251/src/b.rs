use std::collections::HashSet;

use proconio::{input, marker::Chars};
use itertools::Itertools;
type VS = Vec<String>;
pub fn main() {
    input !{
        n: usize,
        w: u32,
        an: [u32; n]
    };
    let mut ans = HashSet::<u32>::new();
    for a in an.clone() {
        if a <= w {
            ans.insert(a);
        }
    }
    for c in an.clone().iter().combinations(2) {
        if c[0] + c[1] <= w {
            ans.insert(c[0] + c[1]);
        }
    }
    for c in an.iter().combinations(3) {
        if c[0] + c[1] + c[2] <= w {
            ans.insert(c[0] + c[1] + c[2]);
        }
    }
    println!("{}", ans.len());
}
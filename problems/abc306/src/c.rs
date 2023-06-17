use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        an: [i32; 3*n]
    };
    let mut searched = HashSet::<i32>::new();
    let mut ans = vec![];
    for i in 0..3*n {
        if searched.contains(&an[i]) {
            ans.push((an[i]).to_string());
            searched.remove(&an[i]);
        } else {
            searched.insert(an[i]);
        }
    }
    println!("{}", ans.iter().join(" "));
}
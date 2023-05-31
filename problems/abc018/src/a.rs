use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::HashMap;
pub fn main() {
    input! {
        abc: [i32; 3]
    };
    let mut sorted = abc.clone();
    sorted.sort();
    let mut ans = HashMap::<i32, usize>::new();
    
    for (rank, i) in sorted.iter().enumerate() {
        ans.insert(*i, 3 - rank);
    }
    for i in abc {
        println!("{}", ans.get(&i).unwrap());
    }
}
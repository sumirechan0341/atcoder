use std::collections::HashMap;
use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut friendmap = HashMap::<Vec<usize>, bool>::new();
    for c in (1..n+1).combinations(2) {
        friendmap.insert(c, false);
    }
    for _i in 0..m {
        input! {
            k: usize,
            xk: [usize; k],
        };
        for c in xk.into_iter().combinations(2) {
            friendmap.insert(c, true);
        }
    }
    for (_, v) in friendmap {
        if !v {
            println!("{}", "No");
            return;
        }
    } 
    println!("{}", "Yes");
}
use std::collections::HashSet;

use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        n: usize,
        mut ln: [i32; n]
    };
    let mut count = 0;
    ln.sort();
    for c in ln.into_iter().combinations(3) {
        if c[0] == c[1] || c[0] == c[2] || c[1] == c[2] {
            continue;
        }
        if ok_triangle(&c) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn ok_triangle(l: &Vec<i32>) -> bool {
    return l[2] < l[0] + l[1];
}
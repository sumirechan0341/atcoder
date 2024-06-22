use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        a2n: [usize; 2*n]
    };
    let mut ans = 0;
    for i in 0..2 * n - 2 {
        if a2n[i] == a2n[i + 2] && a2n[i] != a2n[i + 1] {
            ans += 1;
        }
    }
    println!("{}", ans);
}

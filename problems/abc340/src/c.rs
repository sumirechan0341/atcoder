use std::collections::{HashMap, VecDeque};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize
    };
    let mut memo = HashMap::new();
    solve(n, &mut memo);
    println!("{}", memo[&n]);
}
fn solve(n: usize, memo: &mut HashMap<usize, usize>) {
    if n == 1 {
        memo.insert(1, 0);
        return;
    } else {
    }
    if let Some(&v) = memo.get(&n) {
        return;
    }
    let upper = (n + 1) / 2;
    let lower = n / 2;
    solve(upper, memo);
    solve(lower, memo);
    let upper = memo[&upper];
    let lower = memo[&lower];
    memo.insert(n, upper + lower + n);
}

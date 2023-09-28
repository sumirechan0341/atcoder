use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i64
    };
    let memo = &mut HashMap::<i64, i64>::new();
    println!("{}", f(n, memo));
}

fn f(n: i64, memo: &mut HashMap::<i64, i64>) -> i64 {
    if n == 0 {
        return 1;
    } else {
        let mut right = 0;
        let mut left = 0;
        if let Some(x) = memo.get(&(n/3)) {
            right = *x;
        } else {
            right = f(n/3, memo);
            memo.insert(n/3, right);
        }
        if let Some(x) = memo.get(&(n/2)) {
            left = *x;
        } else {
            left = f(n/2, memo);
            memo.insert(n/2, left);
        }
        return right + left;
    }
}
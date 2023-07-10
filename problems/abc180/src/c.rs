use std::collections::{HashSet, BTreeSet};

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i64
    };
    let mut ans = BTreeSet::<i64>::new();
    for i in 1.. {
        if i*i > n {
            break;
        }
        if n % i == 0 {
            ans.insert(i);
            ans.insert(n/i);
        }
    }
    for a in ans {
        println!("{}", a);
    }
}
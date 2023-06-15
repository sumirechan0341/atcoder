use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        an: [i32; n]
    };
    let mut count = 0;
    let mut ans = HashSet::<i32>::new();
    for a in an {
        if ans.contains(&a) {
            count += 1;
        }
        ans.insert(a);
    }
    println!("{}", count);
}
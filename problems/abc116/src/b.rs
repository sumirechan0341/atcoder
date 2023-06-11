use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        mut s: i32
    };
    let mut searched = HashSet::<i32>::new();
    searched.insert(s);
    for i in 2.. {
        if s % 2 == 0 {
            s /= 2;
        } else {
            s = 3*s + 1;
        }
        if searched.contains(&s) {
            println!("{}", i);
            return;
        } else {
            searched.insert(s);
        }
    }
}
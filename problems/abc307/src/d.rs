use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        s: Chars
    };
    let mut queue = VecDeque::<char>::new();
    let mut open = 0;
    for i in 0..n {
        queue.push_back(s[i]);
        if s[i] == '(' {
            open += 1;
        }
        if s[i] == ')' {
            if open > 0 {
                while Some('(') != queue.pop_back() {}
                open -= 1;
            }
        }
    }
    println!("{}", queue.iter().join(""));
}
use std::collections::VecDeque;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        s: Chars
    };
    let mut copy = VecDeque::new();
    copy.push_front('b');
    if copy == s {
        println!("{}", 0);
        return;
    }
    for i in 1.. {
        if i % 3 == 1 {
            copy.push_front('a');
            copy.push_back('c');
        } else if i % 3 == 2 {
            copy.push_front('c');
            copy.push_back('a');
        } else {
            copy.push_front('b');
            copy.push_back('b');
        }
        if copy == s {
            println!("{}", i);
            return
        }
        if copy.len() > s.len() {
            println!("{}", -1);
            return;
        }
    }
}
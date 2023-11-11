use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    let mut queue = VecDeque::new();
    let mut status = 'A';
    for i in 0..s.len() {
        queue.push_back(s[i]);
        if s[i] == 'C' {
            if queue.len() >= 3 {
                let maybe_c = queue.pop_back().unwrap();
                let maybe_b = queue.pop_back().unwrap();
                let maybe_a = queue.pop_back().unwrap();
                if maybe_a == 'A' && maybe_b == 'B' {

                } else {
                    queue.push_back(maybe_a);
                    queue.push_back(maybe_b);
                    queue.push_back(maybe_c);
                }
            }
        }
    }
    let ans = queue.iter().collect::<Vec<_>>();
    for i in 0..ans.len() {
        print!("{}", ans[i])
    }
    println!("{}", "");
}
use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut pos = VecDeque::new();
    for i in 0..n {
        pos.push_back(((i + 1) as i32, 0));
    }
    let mut ans = vec![];
    for _ in 0..q {
        input! {
            id: usize,
        }
        if id == 1 {
            input! {
                c: char
            }
            if c == 'R' {
                pos.pop_back();
                pos.push_front((pos.get(0).unwrap().0 + 1, pos.get(0).unwrap().1));
            }
            if c == 'L' {
                pos.pop_back();
                pos.push_front((pos.get(0).unwrap().0 - 1, pos.get(0).unwrap().1));
            }
            if c == 'U' {
                pos.pop_back();
                pos.push_front((pos.get(0).unwrap().0, pos.get(0).unwrap().1 + 1));
            }
            if c == 'D' {
                pos.pop_back();
                pos.push_front((pos.get(0).unwrap().0, pos.get(0).unwrap().1 - 1));
            }
        } else {
            input! {
                p: usize
            }
            ans.push(format!(
                "{} {}",
                pos.get(p - 1).unwrap().0,
                pos.get(p - 1).unwrap().1
            ));
        }
    }
    println!("{}", ans.join("\n"));
}

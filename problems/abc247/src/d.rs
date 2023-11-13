use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        q: usize,
    };
    let mut queue = VecDeque::new();
    let mut ans = vec![];
    for i in 0..q {
        input! {
            id: i32
        }
        if id == 1 {
            input! {
                x: i64,
                c: i64
            }
            queue.push_back((x, c));
        } else {
            input! {
                c: i64
            }
            let mut acc = 0;
            let mut local = 0;
            loop {
                if let Some((x, cnow)) = queue.pop_front() {
                    acc += cnow;
                    local += x*cnow;
                    if acc > c {
                        queue.push_front((x, acc-c));
                        local -= (acc-c)*x
                    }
                }
                if acc >= c {
                    break;
                }
            }
            ans.push(local.to_string());
        }
    }
    println!("{}", ans.join("\n"));
}
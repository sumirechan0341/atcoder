use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: i32,
        s: Chars
    };
    let mut path = HashSet::<(i32, i32)>::new();
    let mut now = (0, 0);
    path.insert(now);
    for c in s {
        match c {
            'R' => {
                now = (now.0 + 1, now.1);
            },
            'L' => {
                now = (now.0 - 1, now.1);
            },
            'U' => {
                now = (now.0, now.1 + 1);
            },
            _ => {
                now = (now.0, now.1 - 1);
            },
        }
        if path.contains(&now) {
            println!("{}", "Yes");
            return;
        } else {
            path.insert(now);
        }
    }
    println!("{}", "No");
}
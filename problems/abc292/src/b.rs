use proconio::{input, marker::Chars};
use std::collections::HashMap;
pub fn main() {
    input! {
        n: usize,
        q: usize,
        event: [[i32; 2]; q]
    };
    let mut status: HashMap<i32, i32> = HashMap::new();
    for e in event {
        match e[..] {
            [1, x] => {
                match status.get(&x) {
                    None => { status.insert(x, 1); },
                    Some(1) => { status.insert(x, 2); },
                    _ => { }
                }
            }
            [2, x] => {
                status.insert(x, 2);
            }
            [3, x] => {
                match status.get(&x) {
                    Some(2) => { println!("{}", "Yes"); },
                    _ => { println!("{}", "No"); }
                }
            },
            _ => { println!("{}", "dead_code"); }
        }
    }
}
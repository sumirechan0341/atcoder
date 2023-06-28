use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        x: usize,
        abn: [(usize, usize); n]
    };
    let mut now = vec![false; x+1];
    now[0] = true;
    for ab in abn {
        let mut update = vec![];
        for i in 0..x+1 {
            if now[i] {
                if i + ab.0 < x+1 {
                    update.push(i+ab.0);
                }
                if i + ab.1 < x+1 {
                    update.push(i+ab.1);
                }                
                now[i] = false;
            }
        }
        for i in update {
            now[i] = true;
        }
    }
    println!("{}", if now[x] { "Yes" } else { "No" });
}
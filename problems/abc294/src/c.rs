use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        m: usize,
        an: [i32; n],
        bm: [i32; m]
    };
    let mut ansa = vec![];
    let mut ansb = vec![];
    let mut prev = 1;
    for i in 0..n {
        match bm.binary_search(&an[i]) {
            Ok(x) => { 
                // unreachable
            },
            Err(x) => {
                ansa.push((i+x+1).to_string());
            }
        }
    }

    for i in 0..m {
        match an.binary_search(&bm[i]) {
            Ok(x) => { 
                // unreachable
            },
            Err(x) => {
                ansb.push((i+x+1).to_string());
            }
        }
    }
    
    println!("{}", ansa.join(" "));
    println!("{}", ansb.join(" "));
}
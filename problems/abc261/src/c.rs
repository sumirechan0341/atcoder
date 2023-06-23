use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        sn: [String; n]
    };
    let mut name = HashMap::<String, i32>::new();
    let mut ans = vec![];
    for s in sn {
        match name.get_mut(&s) {
            Some(x) => {
                ans.push(s + "(" + &*x.to_string() + ")");
                *x += 1;
            },
            None => {
                ans.push(s.clone());
                name.insert(s, 1);
            }
        }
    }
    for a in ans {
        println!("{}", a);
    }    
}
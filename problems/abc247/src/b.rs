use std::collections::HashMap;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        sn: [[String; 2]; n]
    };
    let mut nset = HashMap::<String, i32>::new();
    for s in &sn {
        match nset.get(&s[0]) {
            Some(v) => {
                nset.insert(s[0].clone(), v+1);
            },
            None => {
                nset.insert(s[0].clone(), 1);
            }
        };
        if s[0] == s[1] {
            continue;
        }
        match nset.get(&s[1]) {
            Some(v) => {
                nset.insert(s[1].clone(), v+1);
            },
            None => {
                nset.insert(s[1].clone(), 1);
            }
        };
    }
    let valid = nset.iter().filter_map(|x| if x.1 == &1 { Some(x.0.clone()) } else { None }).collect::<Vec<_>>();
    for i in 0..n {
        if valid.contains(&sn[i][0]) || valid.contains(&sn[i][1]) {
            continue;
        } else {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}
use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [[usize; 2]; m]
    };
    let mut connected_map = HashMap::<usize, Vec<usize>>::new();
    for i in 0..m {
        match connected_map.get_mut(&abm[i][0]) {
            Some(v) => {
                v.push(abm[i][1]);
            }
            None => {
                connected_map.insert(abm[i][0], vec![abm[i][1]]);
            }
        };
        match connected_map.get_mut(&abm[i][1]) {
            Some(v) => {
                v.push(abm[i][0]);
            }
            None => {
                connected_map.insert(abm[i][1], vec![abm[i][0]]);
            }
        };
    }
    for i in 0..n {
        match connected_map.get_mut(&(i+1)) {
            None => {
                println!("{}", 0);
            },
            Some(v) => {
                v.sort();
                println!("{} {}", v.len(), v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
            }
        }
    }
}
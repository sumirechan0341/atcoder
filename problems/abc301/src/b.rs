use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut a: [i32; n]
    };
    let mut insert_vec_map: HashMap<usize, Vec<i32>> = HashMap::new();
    for i in 0..n-1 {
        if (a[i] - a[i + 1]).abs() > 1 {
            let mut insert_vec = vec![];
            if a[i + 1] > a[i] {
                for j in a[i]+1..a[i+1] {
                    insert_vec.push(j);
                }
            } else {
                for j in (a[i+1]+1..a[i]).rev() {
                    insert_vec.push(j);
                }
            }
            insert_vec_map.insert(i, insert_vec);
        }
    }
    let mut ans = vec![];
    for i in 0..n {
        ans.push(a[i]);
        match insert_vec_map.get(&i) {
            Some(v) => {
                for e in v {
                    ans.push(*e);
                }
            },
            None => {

            }
        }
    }
    println!("{}", ans.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(" "));
}
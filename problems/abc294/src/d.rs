use std::collections::{BTreeSet};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        q: usize
    };
    let mut waited = BTreeSet::<usize>::new();
    let mut counter = 1;
    let mut ans = vec![];
    for i in 0..q {
        input! {
            event_id: i32
        }
        if event_id == 1 {
            waited.insert(counter);
            counter += 1;
        }
        if event_id == 2 {
            input! {
                x: usize
            }
            waited.remove(&x);
        }
        if event_id == 3 {
            let a = waited.first().unwrap();
            ans.push(a.clone());
        }
    }
    for a in ans {
        println!("{}", a);
    }
}
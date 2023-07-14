use std::collections::{VecDeque, HashMap};

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        hn: [i32; n],
        abm: [(usize, usize); m]
    };
    let mut bridge = HashMap::<usize, i32>::new();
    for (a, b) in abm {
        bridge.entry(a).and_modify(|x| if *x < hn[b-1] { *x = hn[b-1] }).or_insert(hn[b-1]);
        bridge.entry(b).and_modify(|x| if *x < hn[a-1] { *x = hn[a-1] }).or_insert(hn[a-1]);
    }
    let mut count = 0;
    for i in 0..n {
        if let Some(highest_neighbor) = bridge.get(&(i+1)) {
            if highest_neighbor < &hn[i] {
                count += 1;
            }
        } else {
            count += 1;
        }
    }
    println!("{}", count);
}
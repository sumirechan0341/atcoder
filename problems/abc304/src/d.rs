use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        w: usize,
        h: usize,
        n: usize,
        mut pqn: [(usize, usize); n],
        a: usize,
        mut aa: [usize; a],
        b: usize,
        mut bb: [usize; b]
    };
    pqn.sort();
    let mut group = vec![];
    let mut cursor = 0;
    aa.push(w);
    for i in 0..a+1 {
        let mut local = vec![];
        while cursor < n && pqn[cursor].0 < aa[i] {
            local.push(pqn[cursor].1);
            cursor += 1;
        }
        group.push(local);
    }
    bb.sort();
    let mut min = n as i32;
    let mut max = 0;
    for g in group {
        let mut count_map = HashMap::<usize, i32>::new();
        for i in g {
            let pos = bb.partition_point(|&x| x < i);
            count_map.entry(pos).and_modify(|x| *x += 1).or_insert(1);
        }
        if count_map.len() != b+1 {
            min = 0;
        } else if *count_map.values().min().unwrap() < min {
            min = *count_map.values().min().unwrap()
        }
        if count_map.len() != 0 && *count_map.values().max().unwrap() > max {
            max = *count_map.values().max().unwrap();
        }
    }
    println!("{} {}", min, max);
}

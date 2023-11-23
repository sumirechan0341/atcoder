use std::collections::{BTreeSet, BinaryHeap};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut lrn: [(usize, usize); n]
    };
    lrn.sort();
    let mut intervals: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    for (l, r) in lrn {
        if intervals.len() > 0 {
            let x = intervals.peek().unwrap();
            if x.0 <= l && l <= x.1 {
                let mut x = intervals.peek_mut().unwrap();
                x.1 = r.max(x.1);
            } else {
                intervals.push((l, r));
            }
        } else {
            intervals.push((l, r));
        }
    }
    for (l, r) in intervals.into_sorted_vec() {
        println!("{} {}", l, r);
    }
}

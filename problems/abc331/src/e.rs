use itertools::izip;
use proconio::{input, marker::Chars};
use std::{cmp::Reverse, collections::HashSet};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        mut an: [i64; n],
        mut bm: [i64; m],
        cdl: [(usize, usize); l]
    };
    let mut ian = izip!(1.., an).collect::<Vec<_>>();
    let mut ibm = izip!(1.., bm).collect::<Vec<_>>();
    ian.sort_by(|x, y| y.1.cmp(&x.1).then(x.0.cmp(&y.0)));
    ibm.sort_by(|x, y| y.1.cmp(&x.1).then(x.0.cmp(&y.0)));
    let mut map = HashSet::new();
    for (c, d) in cdl {
        map.insert((c, d));
    }
    let mut max = 0;
    for i in 0..n {
        for j in 0..m {
            if map.contains(&(ian[i].0, ibm[j].0)) {
                continue;
            }
            if max < ian[i].1 + ibm[j].1 {
                max = ian[i].1 + ibm[j].1;
            }
            break;
        }
    }
    println!("{}", max);
}

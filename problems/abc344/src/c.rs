use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n],
        m: usize,
        bm: [usize; m],
        l: usize,
        cl: [usize; l],
        q: usize,
        xq: [usize; q]
    };
    let mut set = HashSet::new();
    for i in 0..n {
        for j in 0..m {
            for k in 0..l {
                set.insert(an[i] + bm[j] + cl[k]);
            }
        }
    }
    for i in 0..q {
        if set.contains(&xq[i]) {
            println!("{}", "Yes");
        } else {
            println!("{}", "No");
        }
    }
}

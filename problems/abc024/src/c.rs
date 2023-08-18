use std::collections::{HashMap, HashSet};
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        n: usize,
        rcn: [(usize, usize); n]
    };
    let mut rnum = vec![0; r+1];
    let mut cnum = vec![0; c+1];
    for (i, j) in &rcn {
        rnum[*i] += 1;
        cnum[*j] += 1;
    }
    let mut rk = vec![0; k+1];
    let mut ck = vec![0; k+1];

    for i in 1..r+1 {
        if rnum[i] > k {
            continue;
        }
        rk[rnum[i]] += 1;
    }
    for i in 1..c+1 {
        if cnum[i] > k {
            continue;
        }
        ck[cnum[i]] += 1;
    }
    let mut total: i64 = 0;
    for i in 0..k+1 {
        total += rk[i]*ck[k-i];
    }

    for (i, j) in &rcn {
        if rnum[*i]+cnum[*j] == k {
            total -= 1;
        }
        if rnum[*i]+cnum[*j] == k+1 {
            total += 1;
        }
    }
    println!("{}", total);
}
use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        a: usize,
        tn: [usize; n]
    };
    let mut q = VecDeque::new();
    let mut cur = 0;
    let mut remain = 0;
    let mut status = false;
    for i in 0..=tn[n - 1] + n * a {
        if cur < n && tn[cur] == i {
            q.push_back(i);
            cur += 1;
        }
        if remain > 0 {
            remain -= 1;
        }
        if remain == 0 && status {
            println!("{}", i);
            status = false;
        }
        if remain == 0 {
            if let Some(x) = q.pop_front() {
                remain = a;
                status = true;
            }
        }
    }
}

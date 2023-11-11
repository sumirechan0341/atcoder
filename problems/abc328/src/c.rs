use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lrq: [(usize, usize); q]
    };
    let mut sn = vec![0; n];
    for i in 0..n-1 {
        sn[i+1] += sn[i] + if s[i+1] == s[i] { 1 } else { 0 };
    }
    for (l, r) in lrq {
        println!("{}", sn[r-1]-sn[l-1]);
    }
}
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
        if s[i] == 'A' && s[i+1] == 'C' {
            sn[i+1] = sn[i] + 1;
        } else {
            sn[i+1] = sn[i];
        }
    }
    for (l, r) in lrq {
        println!("{}", sn[r-1]-sn[l-1]);
    }
}
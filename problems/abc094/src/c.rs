use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        xn: [i32; n]
    };
    let mut xnc = xn.clone();
    xnc.sort();
    let median_prev = xnc[n/2-1];
    let median_next = xnc[n/2];
    for b in 0..n {
        if xn[b] > median_prev {
            println!("{}", median_prev);
        } else {
            println!("{}", median_next);
        }
    }
}
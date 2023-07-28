use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        xn: [i64; n]
    };
    let mut min = std::i64::MAX;
    for i in 0..n-k+1 {
        let d = if xn[i] * xn[i+k-1] < 0 {
            if xn[i].abs() > xn[i+k-1].abs() {
                2*xn[i+k-1].abs() + xn[i].abs()
            } else {
                2*xn[i].abs() + xn[i+k-1].abs()
            }
        } else {
            xn[i].abs().max(xn[i+k-1].abs())
        };
        if min > d {
            min = d;
        }
    }
    println!("{}", min);
}
use std::f64::consts::PI;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut rn: [i32; n]
    };
    rn.sort();
    rn.reverse();
    let mut total = 0.0;
    for i in 0..n {
        if i % 2 == 0 {
            total += rn[i] as f64*rn[i] as f64;
        } else {
            total -= rn[i] as f64*rn[i] as f64;
        }
    }
    println!("{}", total*PI);
}
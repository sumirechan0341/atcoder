use proconio::{input, marker::Chars};
use itertools::Itertools;
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        xyn: [(f32, f32); n]
    };
    let mut count = 0;
    for c in (0..n).combinations(2) {
        if lean(xyn[c[0]], xyn[c[1]]).abs() <= 1.0 {
            count += 1;
        }
    }
    println!("{}", count);
}

fn lean(p1: (f32, f32), p2: (f32, f32)) -> f32 {
   return (p1.1 - p2.1) / (p1.0 - p2.0);
}
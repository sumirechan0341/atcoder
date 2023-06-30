use itertools::Itertools;
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        xyn: [(i64, i64); n]
    };
    let mut count = 0;
    for c in (0..n).combinations(3) {
        let v1 = (xyn[c[0]].0-xyn[c[1]].0, xyn[c[0]].1-xyn[c[1]].1);
        let v2 = (xyn[c[0]].0-xyn[c[2]].0, xyn[c[0]].1-xyn[c[2]].1);
        if v1.0*v2.1 - v1.1*v2.0 != 0 {
            count += 1;
        }
    }
    println!("{}", count);
}
use proconio::{input, marker::Chars};
use itertools::Itertools;
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        xyn: [(i32, i32); n]
    };
    for c in (0..n).combinations(3) {
        let p1 = (xyn[c[1]].0 - xyn[c[0]].0, xyn[c[1]].1 - xyn[c[0]].1);
        let p2 = (xyn[c[2]].0 - xyn[c[0]].0, xyn[c[2]].1 - xyn[c[0]].1);
        if p1.0 * p2.1 == p1.1 * p2.0 {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}
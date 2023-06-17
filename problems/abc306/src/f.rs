use proconio::{input, marker::Chars};
use itertools::Itertools;
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        m: usize,
        anm: [[i64; m]; n]
    };
    let mut total = 0;
    for c in (0..n).combinations(2) {
        let mut a1 = anm[c[0]].clone();
        a1.sort();
        a1.reverse();
        let mut a2 = anm[c[1]].clone();
        a2.sort();
        for a in a1 {
            match a2.binary_search(&a) {
                Ok(x) => { println!("{}", "unreachable"); },
                Err(x) => { total += x+1 }
            }
        }
    }
    println!("{}", total);
}
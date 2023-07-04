use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        n: usize,
        tlrn: [(i64, i64, i64); n],        
    };
    let mut count = 0;
    for c in (0..n).combinations(2) {
        let s1 = tlrn[c[0]];
        let s2 = tlrn[c[1]];
        if s1.2 == s2.1 && s1.0 % 2 == 1 && s2.0 <= 2 {
            // s1][s2
            count += 1;
            continue;
        }
        if s1.1 == s2.2 && s1.0 <= 2 && s2.0 % 2 == 1 {
            // s2][s1
            count += 1;
            continue;
        }
        if s1.2 > s2.1 && s1.1 < s2.2 {
            // [ s1 [ ] s2 ]
            count += 1;
            continue;
        }
        if s1.1 > s2.2 && s1.2 < s2.1 {
            // [ s2 [ ] s1 ]
            count += 1;
            continue;
        }
    }
    println!("{}", count);
}
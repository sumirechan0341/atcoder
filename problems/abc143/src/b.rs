use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        n: usize,
        dn: [i32; n]
    };
    let mut total = 0;
    for c in (0..n).combinations(2) {
        total += dn[c[0]] * dn[c[1]]
    }
    println!("{}", total);
}
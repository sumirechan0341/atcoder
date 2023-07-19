use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        n: usize,
        xyn: [(f64, f64); n]
    };
    let mut total = 0.0;
    for perm in (0..n).permutations(n) {
        for i in 0..n-1 {
            total += ((xyn[perm[i]].0 - xyn[perm[i+1]].0).powf(2.0) + (xyn[perm[i]].1 - xyn[perm[i+1]].1).powf(2.0)).sqrt()
        }
    }
    println!("{}", total / ((0..n).permutations(n).count() as f64));
}
use proconio::{input, marker::Chars};
use std::u64::MAX;
pub fn main() {
    input !{
        n: usize,
        k: usize,
        ak: [usize; k],
        xyn: [(i128, i128); n]
    };
    let mut min_distance: Vec<u64> = vec![MAX; n];
    for a in ak {
        for i in 0..n {
            min_distance[i] = min_distance[i].min(norm2(xyn[a-1], xyn[i]))
        }
    }
    min_distance.sort();
    println!("{}", (min_distance[n-1] as f64).sqrt());
}
fn norm2(p1: (i128, i128), p2: (i128, i128)) -> u64 {
    return ((p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2)) as u64;
}
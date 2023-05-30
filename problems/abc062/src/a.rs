use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        x: i32,
        y: i32
    };
    let g1: itertools::Permutations<std::slice::Iter<i32>> = [1, 3, 5, 7, 8, 10, 12].iter().permutations(2);
    let g2: itertools::Permutations<std::slice::Iter<i32>> = [4, 6, 9, 11].iter().permutations(2);
    println!("{}", if g1.collect::<Vec<_>>().contains(&vec![&x, &y]) || g2.collect::<Vec<_>>().contains(&vec![&x, &y]) { "Yes" } else { "No" });
}
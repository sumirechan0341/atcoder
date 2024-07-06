use std::collections::BinaryHeap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        mut an: [usize; n]
    };
    an.sort();
    println!(
        "{}",
        an.windows(n - k)
            .map(|x| x.last().unwrap() - x.first().unwrap())
            .min()
            .unwrap()
    );
}

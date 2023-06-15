use itertools::Itertools;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        l: usize,
        mut sn: [Chars; n]
    };
    sn.sort();
    println!("{}", sn.iter().map(|cv| cv.iter().collect::<String>()).join(""));
}
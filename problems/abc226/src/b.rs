use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
    };
    let mut aset = HashSet::<Vec<i32>>::new();
    for _ in 0..n {
        input! {
            l: usize,
            al: [i32; l]
        }
        aset.insert(al);
    }
    println!("{}", aset.len());
}
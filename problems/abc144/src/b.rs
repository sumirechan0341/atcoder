use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32
    };
    let mut kuku = HashSet::<i32>::new();
    for i in 1..10 {
        for j in 1..10 {
            kuku.insert(i * j);
        }
    }
    println!("{}", if kuku.contains(&n) { "Yes" } else { "No" });
}
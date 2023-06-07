use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String
    };
    let mut sset = HashSet::<String>::new();
    sset.insert("ABC".to_string());
    sset.insert("ARC".to_string());
    sset.insert("AGC".to_string());
    sset.insert("AHC".to_string());

    sset.remove(&s1);
    sset.remove(&s2);
    sset.remove(&s3);
    println!("{}", sset.iter().collect::<Vec<_>>()[0]);
}
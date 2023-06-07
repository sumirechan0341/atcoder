use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        stn: [(String, String); n]
    };
    let mut names = HashSet::<(String, String)>::new();
    for st in &stn {
        names.insert(st.clone());
    }
    println!("{}", if names.iter().len() == stn.len() { "No" } else { "Yes" });
}
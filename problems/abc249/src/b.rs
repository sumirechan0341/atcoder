use proconio::{input, marker::Chars};
use std::collections::HashSet;
use std::iter::FromIterator;
type VS = Vec<String>;

pub fn main() {
    input !{
        s: Chars
    };
    let sset = HashSet::<char>::from_iter(s.clone());
    if sset.len() != s.len() {
        println!("{}", "No");
        return;
    }
    if s.iter().any(|c| c.is_uppercase()) && s.iter().any(|c| c.is_lowercase()) {
        println!("{}", "Yes");
        return;
    }
    println!("{}", "No");
}
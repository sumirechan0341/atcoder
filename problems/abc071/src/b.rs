use std::collections::HashSet;
use std::iter::FromIterator;
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        s: Chars
    };
    let set = HashSet::<char>::from_iter(s);
    for i in 97_u8..123 {
        if !set.contains(&(i as char)) {
            println!("{}", i as char);
            return;
        }
    }
    println!("{}", "None");
}
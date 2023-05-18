use proconio::{input, marker::Chars};
use std::collections::HashSet;
use std::iter::FromIterator;
pub fn main() {
    input! {
        s: Chars
    };
    let set: HashSet<&char> = HashSet::from_iter(s.iter());
    println!("{}", if set.len() == 1 { 1 } else if set.len() == 2 { 3 } else { 6 });
}
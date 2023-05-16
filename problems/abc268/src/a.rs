use proconio::input;
use std::collections::HashSet;
use std::iter::FromIterator;
pub fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32
    }
    let set: HashSet<i32> = HashSet::from_iter(vec![a, b, c, d, e]);
    println!("{}", set.len());
}
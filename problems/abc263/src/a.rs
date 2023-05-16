use proconio::input;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize
    };
    let group: HashSet<usize> = HashSet::from_iter(vec![a, b, c, d, e]);
    let acounter = [a, b, c, d, e].iter().filter(|&x| *x == a).count();
    println!("{}", if group.len() == 2 && (acounter == 2 || acounter == 3) {"Yes"} else {"No"});
}
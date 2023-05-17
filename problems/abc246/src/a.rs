use proconio::input;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn main() {
    input! {
        xy1: [i32; 2],
        xy2: [i32; 2],
        xy3: [i32; 2],
    };
    let xset: HashSet<i32> = HashSet::from_iter(vec![xy1[0], xy2[0], xy3[0]]);
    let yset: HashSet<i32> = HashSet::from_iter(vec![xy1[1], xy2[1], xy3[1]]);
    println!("{} {}", xset.iter().sum::<i32>() * 2 - (xy1[0] + xy2[0] + xy3[0]), yset.iter().sum::<i32>() * 2 - (xy1[1] + xy2[1] + xy3[1]));
}
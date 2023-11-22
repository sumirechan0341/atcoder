use itertools::Itertools;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    println!("{}", s.iter().join(" "));
}
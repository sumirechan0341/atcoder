use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        n: usize,
        m: usize
    };
    for c in (1..=m).combinations(n) {
        println!("{}", c.iter().map(|x| x.to_string()).join(" "));
    }
}
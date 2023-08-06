use proconio::{input, marker::Chars};
use num::Integer;
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        tn: [i64; n]
    };
    println!("{}", tn.iter().fold(1, |acc, x| acc.lcm(x)));
}
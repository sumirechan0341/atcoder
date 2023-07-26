use proconio::{input, marker::Chars};
use num::Integer;
pub fn main() {
    input! {
        n: usize,
        an: [i32; n]
    };
    println!("{}", an.iter().fold(an[0], |acc, x| acc.gcd(x)));
}
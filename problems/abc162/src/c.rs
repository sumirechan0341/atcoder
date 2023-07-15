use proconio::{input, marker::Chars};
use num::integer::gcd;
pub fn main() {
    input! {
        k: i32
    };
    let mut total = 0;
    for a in 1..=k {
        for b in 1..=k {
            for c in 1..=k {
                total += gcd(gcd(a, b), c)
            }
        }
    }
    println!("{}", total);
}
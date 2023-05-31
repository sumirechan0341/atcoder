use proconio::{input, marker::Chars};
use num::Rational;
pub fn main() {
    input! {
        a: Rational,
        b: Rational,
        c: Rational,
        d: Rational
    };
    println!("{}", if b / a > d / c { "TAKAHASHI" } else if b / a == d / c { "DRAW" } else { "AOKI" });
}
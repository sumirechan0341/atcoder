use proconio::{input, marker::Chars};
use num::Rational32;
pub fn main() {
    input! {
        a: Rational32,
        b: Rational32,
        c: Rational32,
        d: Rational32
    };
    println!("{}", if b / a > d / c { "TAKAHASHI" } else if b / a == d / c { "DRAW" } else { "AOKI" });
}
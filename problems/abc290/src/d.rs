use proconio::{input, marker::Chars};
use num::Integer;
pub fn main() {
    input! {
        t: usize,
        testt: [(i64, i64, i64); t]
    };
    for (n, d, k) in testt {
        let order = n.lcm(&d)/d;
        println!("{}", (((k-1)/order)+d*((k-1)%order))%n);
    }
}
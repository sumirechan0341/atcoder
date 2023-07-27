use proconio::{input, marker::Chars};
use num::Integer;
pub fn main() {
    input! {
        n: usize,
        x: i32,
        xn: [i32; n]
    };
    let mut gcd = (x-xn[0]).abs();
    for i in 0..n-1 {
        gcd = gcd.gcd(&((xn[i+1]-xn[i]).abs()));
    }
    println!("{}", gcd);
}
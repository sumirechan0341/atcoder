use num::Integer;
use proconio::{input, marker::Chars};
pub fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32
    };
    if c == 0 {
        println!("{}", if a % b == 0 || b % a == 0 { "YES" } else { "NO" });
        return;
    }
    if a.gcd(&b) == 1 {
        println!("{}", "YES");
        return;
    }
    if a.gcd(&b) != 1 {
        println!("{}", if c % a.gcd(&b) == 0 { "YES" } else { "NO" });
        return;
    }
    println!("{}", "NO");
}

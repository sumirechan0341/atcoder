use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i64,
        b: i64
    };
    println!("{}", (a*b) / gcd(a, b));
}

fn gcd(a: i64, b: i64) -> i64 {
    let r = a.max(b) % a.min(b);
    if r == 0 {
        return a.min(b);
    }
    return gcd(a.min(b), r);
}
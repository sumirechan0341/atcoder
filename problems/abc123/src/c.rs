use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
    };
    println!("{}", 5 + (n-1)/a.min(b).min(c).min(d).min(e));
}
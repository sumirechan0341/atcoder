use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i64,
        b: i64,
        k: i64,
        l: i64
    };
    if a*l < b {
        println!("{}", a*k);
    } else {
        println!("{}", k/l*b+(a*(k%l)).min(b));
    }
}
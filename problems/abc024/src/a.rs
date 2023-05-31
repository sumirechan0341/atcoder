use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        k: i32,
        s: i32,
        t: i32
    };
    println!("{}", a * s + b * t - if s + t >= k { s + t } else { 0 } * c);
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        k: i32
    };
    println!("{}", if k <= a { k } else if k <= a + b { a } else { a - (k - a - b).min(c) });
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32
    };
    println!("{}", if b == 0 && a == c { "?" } else if c == a + b { "+" } else if c == a - b { "-" } else { "!" });
}
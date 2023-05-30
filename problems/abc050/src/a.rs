use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32,
        op: char,
        b: i32
    };
    println!("{}", if op == '+' { a + b } else { a - b });
}
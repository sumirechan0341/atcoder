use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32,
        d: i32
    };
    println!("{}", ((a + 1) * d).max((d + 1) * a));
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        w: i32,
        a: i32,
        b: i32
    };
    println!("{}", ((b-a).abs()-w).max(0));
}
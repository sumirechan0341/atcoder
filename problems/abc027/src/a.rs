use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        l1: i32,
        l2: i32,
        l3: i32
    };
    println!("{}", l1 ^ l2 ^ l3);
}
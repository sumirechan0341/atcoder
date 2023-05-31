use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        x: char
    };
    println!("{}", x as u8 - 'A' as u8 + 1);
}
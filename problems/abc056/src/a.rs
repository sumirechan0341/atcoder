use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: char,
        b: char
    };
    println!("{}", if (a as u8 ^ b as u8) == 0 { 'H' } else { 'D' });
}
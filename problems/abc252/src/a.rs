use proconio::input;
use std::char;
pub fn main() {
    input! {
        n: u8
    };
    println!("{}", char::from(n));
}
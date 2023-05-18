use proconio::input;
use proconio::marker::Chars;
pub fn main() {
    input! {
        abc: Chars
    };
    println!("{}", abc.iter().map(|c| c.to_digit(10).unwrap() * 111).sum::<u32>());
}
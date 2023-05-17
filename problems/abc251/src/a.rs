use proconio::input;
use proconio::marker::Chars;
pub fn main() {
    input! {
        s: Chars
    };
    println!("{}", s.repeat(6)[0..6].iter().collect::<String>());    
}
use proconio::input;
use proconio::marker::Chars;
pub fn main() {
    input !{
        _n: usize,
        s: Chars
    };
    println!("{}", s[s.len()-1]);
}
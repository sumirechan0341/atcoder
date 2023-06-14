use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        a: u64,
        b: u64,
        x: u64
    };
    // aの前のxの倍数 (a / x) * x
    // bの前のxの倍数 (b / x) * x 
    println!("{}", ((b / x) * x - (a / x) * x) / x + if a % x == 0 { 1 } else { 0 } );
}
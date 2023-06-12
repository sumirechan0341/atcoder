use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: i32
    };
    let mut a = n;
    let mut s = 0;
    while a != 0 {
        s += a % 10;
        a /= 10; 
    }
    println!("{}", if n % s == 0 { "Yes" } else { "No" });
}
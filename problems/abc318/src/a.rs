use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i32,
        m: i32,
        p: i32
    };
    println!("{}", if n < m { 0 } else {(n-m)/p+1});
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: i32
    };
    println!("{}", if n - 5 * (n / 5) > 2 { 5 * (n / 5 + 1) } else { 5 * (n / 5) } );
}